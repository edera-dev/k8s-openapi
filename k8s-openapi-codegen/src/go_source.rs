//! Fetching and parsing the upstream Go source of Kubernetes API types.
//!
//! The OpenAPI spec omits some information that is present in the Go source, like the values of string enums,
//! and some types entirely, like `meta/v1.Table`. This module provides access to the Go source so that
//! such information can be recovered.
//!
//! Packages are identified by their Go import path relative to `k8s.io/`, like `api/core/v1` or `apimachinery/pkg/apis/meta/v1`.

/// Packages that are always fetched, in addition to the `k8s.io/api` packages that have definitions in the spec.
const EXTRA_PACKAGES: &[&str] = &[
    "apimachinery/pkg/apis/meta/v1",
];

pub(crate) type Packages<'a> = std::collections::BTreeMap<&'a str, Package<'a>>;

/// Fetches the `types.go` file of every package relevant to the given spec.
///
/// Returns a map of package path (like `api/core/v1`) to the file's contents.
pub(crate) async fn fetch(
    definitions: &std::collections::BTreeMap<crate::swagger20::DefinitionPath, crate::swagger20::Schema>,
    go_source_url: &str,
    client: &reqwest::Client,
) -> Result<std::collections::BTreeMap<String, String>, crate::Error> {
    use futures_util::TryStreamExt;

    let packages: std::collections::BTreeSet<_> =
        definitions.keys()
        .filter_map(|definition_path| definition_path.0.strip_prefix("io.k8s.api.")?.rsplit_once('.'))
        .map(|(namespace, _)| format!("api/{}", namespace.replace('.', "/")))
        .chain(EXTRA_PACKAGES.iter().map(|&package_path| package_path.to_owned()))
        .collect();

    log::info!("Fetching Go source of {} packages from {go_source_url} ...", packages.len());

    let sources: futures_util::stream::FuturesUnordered<_> =
        packages.into_iter()
        .map(|package_path| async move {
            let url = format!("{go_source_url}/{package_path}/types.go");
            let response = client.get(&url).send().await?;
            let status = response.status();
            if status != reqwest::StatusCode::OK {
                return Err(format!("{url}: {status}").into());
            }
            let source = response.text().await?;
            Ok::<_, crate::Error>((package_path, source))
        })
        .collect();
    let sources = sources.try_collect().await?;

    log::info!("OK");

    Ok(sources)
}

pub(crate) fn parse(go_sources: &std::collections::BTreeMap<String, String>) -> Result<Packages<'_>, crate::Error> {
    go_sources.iter()
        .map(|(package_path, source)| Ok((&**package_path, parse_package(source).map_err(|err| format!("{package_path}/types.go: {err}"))?)))
        .collect()
}

/// The namespace of the spec definitions corresponding to the given package, like `io.k8s.api.core.v1`
pub(crate) fn definition_namespace(package_path: &str) -> String {
    format!("io.k8s.{}", package_path.replace('/', "."))
}

/// The path of the Rust module corresponding to the given package, like `crate::api::core::v1`
pub(crate) fn rust_module_path(package_path: &str) -> String {
    std::iter::once("crate".into())
        .chain(package_path.split('/').map(k8s_openapi_codegen_common::get_rust_ident))
        .collect::<Vec<_>>()
        .join("::")
}

/// The directory of the Rust module corresponding to the given package
pub(crate) fn rust_module_dir(out_dir: &std::path::Path, package_path: &str) -> std::path::PathBuf {
    let mut dir = out_dir.to_owned();
    for part in package_path.split('/') {
        dir.push(&*k8s_openapi_codegen_common::get_rust_ident(part));
    }
    dir
}

/// The parts of a Go `types.go` file that are relevant to codegen.
#[derive(Default)]
pub(crate) struct Package<'a> {
    /// Import alias => package path
    pub(crate) imports: std::collections::BTreeMap<&'a str, &'a str>,

    /// `type Foo string` declarations
    pub(crate) string_types: std::collections::BTreeMap<&'a str, StringType>,

    /// `type Foo = alias.Bar` and `type Foo = Bar` declarations
    pub(crate) type_aliases: std::collections::BTreeMap<&'a str, TypeAlias<'a>>,

    /// `const` declarations whose values are strings or references to other consts, in declaration order
    pub(crate) consts: Vec<Const<'a>>,

    /// `type Foo struct { ... }` declarations
    pub(crate) structs: std::collections::BTreeMap<&'a str, Struct<'a>>,

    /// The names of `structs`, in declaration order
    pub(crate) struct_names: Vec<&'a str>,
}

pub(crate) struct StringType {
    pub(crate) doc: String,
    pub(crate) is_enum: bool,
}

pub(crate) struct TypeAlias<'a> {
    /// `None` if the aliased type is in the same package
    pub(crate) package: Option<&'a str>,
    pub(crate) type_name: &'a str,
    pub(crate) is_enum: bool,
}

pub(crate) struct Const<'a> {
    pub(crate) name: &'a str,
    pub(crate) type_name: Option<&'a str>,
    pub(crate) value: Expr<'a>,
    pub(crate) doc: String,
}

pub(crate) enum Expr<'a> {
    Literal(&'a str),
    Ident(Option<&'a str>, &'a str),
}

pub(crate) struct Struct<'a> {
    pub(crate) doc: String,
    pub(crate) fields: Vec<Field<'a>>,
}

pub(crate) struct Field<'a> {
    /// The JSON name of the field, or `None` if the field is an embedded struct whose fields are inlined, like `TypeMeta`
    pub(crate) json_name: Option<&'a str>,
    pub(crate) omitempty: bool,
    pub(crate) ty: Type<'a>,
    /// The doc comment, without `+marker` lines
    pub(crate) doc: String,
    /// The `+marker` lines of the doc comment, without the leading `+`
    pub(crate) markers: Vec<&'a str>,
}

#[derive(Debug)]
pub(crate) enum Type<'a> {
    /// A possibly-qualified type name, like `string`, `ObjectMeta` or `metav1.Time`
    Named(Option<&'a str>, &'a str),
    Pointer(Box<Type<'a>>),
    Slice(Box<Type<'a>>),
    Map(Box<Type<'a>>, Box<Type<'a>>),
    Interface,
}

impl<'a> Type<'a> {
    fn parse(s: &'a str) -> Option<Self> {
        if let Some(rest) = s.strip_prefix("[]") {
            Some(Type::Slice(Box::new(Type::parse(rest)?)))
        }
        else if let Some(rest) = s.strip_prefix('*') {
            Some(Type::Pointer(Box::new(Type::parse(rest)?)))
        }
        else if let Some(rest) = s.strip_prefix("map[") {
            let (key, value) = rest.split_once(']')?;
            Some(Type::Map(Box::new(Type::parse(key)?), Box::new(Type::parse(value)?)))
        }
        else if s == "interface{}" || s == "any" {
            Some(Type::Interface)
        }
        else {
            let (Some((alias, name)), "") = split_qualified_ident(s) else { return None };
            Some(Type::Named(alias, name))
        }
    }

    /// The innermost named type, ignoring pointers, slices and map values.
    pub(crate) fn element(&self) -> Option<(Option<&'a str>, &'a str)> {
        match self {
            Type::Named(alias, name) => Some((*alias, name)),
            Type::Pointer(ty) | Type::Slice(ty) | Type::Map(_, ty) => ty.element(),
            Type::Interface => None,
        }
    }
}

/// A line-based parser for the subset of Go used by `types.go` files.
fn parse_package(source: &str) -> Result<Package<'_>, crate::Error> {
    enum State<'a> {
        TopLevel,
        Import,
        Const,
        Struct(&'a str),
    }

    let mut package = Package::default();
    let mut state = State::TopLevel;
    let mut doc: Vec<&str> = vec![];

    for line in source.lines() {
        let trimmed = line.trim();

        if let Some(comment) = trimmed.strip_prefix("//") {
            doc.push(comment.strip_prefix(' ').unwrap_or(comment));
            continue;
        }

        // Doc comments must immediately precede the item they document.
        let doc = std::mem::take(&mut doc);
        if trimmed.is_empty() {
            continue;
        }

        match state {
            State::TopLevel =>
                if trimmed == "import (" {
                    state = State::Import;
                }
                else if let Some(rest) = trimmed.strip_prefix("import ") {
                    parse_import(&mut package, rest);
                }
                else if trimmed == "const (" {
                    state = State::Const;
                }
                else if let Some(rest) = trimmed.strip_prefix("const ") {
                    parse_const(&mut package, rest, &doc);
                }
                else if let Some(rest) = line.strip_prefix("type ") {
                    let mut parts = rest.split_whitespace();
                    let (Some(name), Some(kind)) = (parts.next(), parts.next()) else { continue };
                    if kind == "=" {
                        // Type alias, like `type ScopeType = v1.ScopeType` or `type IPFamilyPolicyType = IPFamilyPolicy`
                        let Some(target) = parts.next() else { continue };
                        let (target_package, type_name) = match target.split_once('.') {
                            Some((alias, type_name)) => {
                                let Some(&target_package) = package.imports.get(alias) else { continue };
                                (Some(target_package), type_name)
                            },
                            None => (None, target),
                        };
                        package.type_aliases.insert(name, TypeAlias { package: target_package, type_name, is_enum: doc.contains(&"+enum") });
                    }
                    else if kind == "string" {
                        let is_enum = doc.contains(&"+enum");
                        let doc = join_doc(&doc);
                        package.string_types.insert(name, StringType { doc, is_enum });
                    }
                    else if kind == "struct" && parts.next() == Some("{") {
                        package.structs.insert(name, Struct { doc: join_doc(&doc), fields: vec![] });
                        package.struct_names.push(name);
                        state = State::Struct(name);
                    }
                },

            State::Import =>
                if trimmed == ")" {
                    state = State::TopLevel;
                }
                else {
                    parse_import(&mut package, trimmed);
                },

            State::Const =>
                if trimmed == ")" {
                    state = State::TopLevel;
                }
                else {
                    parse_const(&mut package, trimmed, &doc);
                },

            State::Struct(struct_name) =>
                if line.starts_with('}') {
                    state = State::TopLevel;
                }
                else if let Some(field) = parse_field(trimmed, &doc) {
                    package.structs.get_mut(struct_name).expect("struct was inserted when its declaration was parsed").fields.push(field);
                },
        }
    }

    // Sanity check that every `+enum` marker was attached to a string type.
    let num_enum_markers = source.lines().filter(|line| line.trim() == "// +enum").count();
    let num_enums =
        package.string_types.values().filter(|string_type| string_type.is_enum).count() +
        package.type_aliases.values().filter(|type_alias| type_alias.is_enum).count();
    if num_enum_markers != num_enums {
        return Err(format!("found {num_enum_markers} +enum markers but {num_enums} enum types").into());
    }

    Ok(package)
}

fn join_doc(doc: &[&str]) -> String {
    let doc: Vec<_> = doc.iter().copied().filter(|line| !line.starts_with('+')).collect();
    doc.join("\n").trim().to_owned()
}

fn parse_import<'a>(package: &mut Package<'a>, s: &'a str) {
    let (alias, path) = match s.split_once(' ') {
        Some((alias, path)) => (Some(alias), path.trim()),
        None => (None, s),
    };
    let Some(path) = path.strip_prefix('"').and_then(|path| path.strip_suffix('"')) else { return };
    let Some(package_path) = path.strip_prefix("k8s.io/") else { return };
    let alias = alias.unwrap_or_else(|| package_path.rsplit('/').next().unwrap_or(package_path));
    package.imports.insert(alias, package_path);
}

/// Parses `Name Type = value`, `Name = Type(value)` and `Name = value`, where value is a string literal or a possibly-qualified const name.
fn parse_const<'a>(package: &mut Package<'a>, s: &'a str, doc: &[&str]) {
    let (name, rest) = split_ident(s);
    if name.is_empty() {
        return;
    }
    let rest = rest.trim_start();

    let (mut type_name, rest) =
        if let Some(rest) = rest.strip_prefix('=') {
            (None, rest)
        }
        else {
            let (type_name, rest) = split_ident(rest);
            if type_name.is_empty() {
                return;
            }
            let Some(rest) = rest.trim_start().strip_prefix('=') else { return };
            (Some(type_name), rest)
        };
    let rest = rest.trim_start();

    let value =
        if let Some(rest) = rest.strip_prefix('"') {
            let Some(end) = rest.find('"') else { return };
            Expr::Literal(&rest[..end])
        }
        else {
            let (ident, rest) = split_qualified_ident(rest);
            let Some(ident) = ident else { return };
            if let Some(rest) = rest.strip_prefix('(') {
                // Conversion like `Type("value")` or `Type(other.Const)`
                let (None, conversion_type_name) = ident else { return };
                if type_name.is_none() {
                    type_name = Some(conversion_type_name);
                }
                if let Some(rest) = rest.strip_prefix('"') {
                    let Some(end) = rest.find('"') else { return };
                    Expr::Literal(&rest[..end])
                }
                else {
                    let (Some((alias, name)), rest) = split_qualified_ident(rest) else { return };
                    if !rest.starts_with(')') {
                        return;
                    }
                    Expr::Ident(alias, name)
                }
            }
            else {
                Expr::Ident(ident.0, ident.1)
            }
        };

    package.consts.push(Const { name, type_name, value, doc: join_doc(doc) });
}

/// Parses a struct field like ``Name *Type `json:"name,omitempty"` ``, or an embedded field like ``TypeMeta `json:",inline"` ``.
fn parse_field<'a>(s: &'a str, doc: &[&'a str]) -> Option<Field<'a>> {
    let (declaration, tag) = s.split_once('`')?;
    let mut parts = declaration.split_whitespace();
    let first = parts.next()?;
    let ty = parts.next().unwrap_or(first);
    let ty = Type::parse(ty)?;

    let (_, json_tag) = tag.split_once("json:\"")?;
    let json_tag = &json_tag[..json_tag.find('"')?];
    let mut json_tag_parts = json_tag.split(',');
    let json_name = json_tag_parts.next()?;
    let json_name = match json_name {
        "-" => return None,
        "" => None,
        json_name => Some(json_name),
    };
    let omitempty = json_tag_parts.any(|option| option == "omitempty");

    let markers = doc.iter().filter_map(|line| line.strip_prefix('+')).collect();

    Some(Field { json_name, omitempty, ty, doc: join_doc(doc), markers })
}

fn split_ident(s: &str) -> (&str, &str) {
    let end = s.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).unwrap_or(s.len());
    s.split_at(end)
}

fn split_qualified_ident(s: &str) -> (Option<(Option<&str>, &str)>, &str) {
    let (first, rest) = split_ident(s);
    if first.is_empty() {
        return (None, s);
    }
    if let Some(rest) = rest.strip_prefix('.') {
        let (second, rest) = split_ident(rest);
        if second.is_empty() {
            return (None, s);
        }
        return (Some((Some(first), second)), rest);
    }
    (Some((None, first)), rest)
}
