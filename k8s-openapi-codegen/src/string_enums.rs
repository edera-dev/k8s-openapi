//! Enums for string fields whose values are one of a fixed set, like phases, policies and modes.
//!
//! The upstream spec types these fields as plain strings and only lists their values in the field's description.
//! However the Go source of the `k8s.io/api` module marks such string types with a `// +enum` comment
//! and defines their values as constants, so the enums are generated from that instead.
//!
//! Each enum is emitted as an invocation of the `string_enum!` macro into the module corresponding to its Go package.

/// String types that are not marked `+enum` upstream because they are open to custom values,
/// but which are still useful to have enums for the well-known values.
const EXTRA_ENUMS: &[(&str, &str)] = &[
    ("core/v1", "ConditionStatus"),
    ("core/v1", "PodConditionType"),
];

/// Fetches the `types.go` file of every `k8s.io/api` package that has definitions in the given spec.
///
/// Returns a map of package path (like `core/v1`) to the file's contents.
pub(crate) async fn fetch_go_sources(
    definitions: &std::collections::BTreeMap<crate::swagger20::DefinitionPath, crate::swagger20::Schema>,
    api_source_url: &str,
    client: &reqwest::Client,
) -> Result<std::collections::BTreeMap<String, String>, crate::Error> {
    use futures_util::TryStreamExt;

    let packages: std::collections::BTreeSet<_> =
        definitions.keys()
        .filter_map(|definition_path| definition_path.0.strip_prefix("io.k8s.api.")?.rsplit_once('.'))
        .map(|(namespace, _)| namespace.replace('.', "/"))
        .collect();

    log::info!("Fetching Go source of {} packages from {api_source_url} ...", packages.len());

    let sources: futures_util::stream::FuturesUnordered<_> =
        packages.into_iter()
        .map(|package| async move {
            let url = format!("{api_source_url}/{package}/types.go");
            let response = client.get(&url).send().await?;
            let status = response.status();
            if status != reqwest::StatusCode::OK {
                return Err(format!("{url}: {status}").into());
            }
            let source = response.text().await?;
            Ok::<_, crate::Error>((package, source))
        })
        .collect();
    let sources = sources.try_collect().await?;

    log::info!("OK");

    Ok(sources)
}

pub(crate) fn generate(
    definitions: &std::collections::BTreeMap<crate::swagger20::DefinitionPath, crate::swagger20::Schema>,
    go_sources: &std::collections::BTreeMap<String, String>,
    out_dir: &std::path::Path,
) -> Result<usize, crate::Error> {
    use std::io::Write;

    let packages: std::collections::BTreeMap<&str, Package<'_>> =
        go_sources.iter()
        .map(|(package_path, source)| Ok((&**package_path, parse_package(source).map_err(|err| format!("{package_path}/types.go: {err}"))?)))
        .collect::<Result<_, crate::Error>>()?;

    for &(package_path, type_name) in EXTRA_ENUMS {
        if !packages.get(package_path).is_some_and(|package| package.string_types.contains_key(type_name)) {
            return Err(format!("extra string enum {package_path}.{type_name} not found").into());
        }
    }

    // The fields of each enum, for the "Used by" section of its docs.
    let mut used_by: std::collections::BTreeMap<(&str, &str), Vec<String>> = Default::default();
    for (&package_path, package) in &packages {
        for field in &package.fields {
            let Some(enum_key) = resolve_enum(&packages, package_path, field.type_package, field.type_name) else { continue };

            let definition_path = format!("io.k8s.api.{}.{}", package_path.replace('/', "."), field.struct_name);
            let Some(definition) = definitions.get(&*definition_path) else { continue };
            let crate::swagger20::SchemaKind::Properties(properties) = &definition.kind else { continue };
            if !properties.contains_key(field.json_name) {
                continue;
            }

            let rust_type_path = format!("crate::api::{}::{}", package_path.replace('/', "::"), field.struct_name);
            let rust_field_name = k8s_openapi_codegen_common::get_rust_ident(field.json_name);
            used_by.entry(enum_key).or_default().push(
                format!("[`{}::{rust_field_name}`]({rust_type_path}::{rust_field_name})", field.struct_name));
        }
    }

    let mut num_generated = 0;

    for (&package_path, package) in &packages {
        let mut dir = out_dir.join("api");
        for part in package_path.split('/') {
            dir.push(&*k8s_openapi_codegen_common::get_rust_ident(part));
        }

        let mut mod_rs = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).open(dir.join("mod.rs"))?);

        for (&type_name, string_type) in &package.string_types {
            if !package.is_enum(package_path, type_name) {
                continue;
            }

            let error = |err: &dyn std::fmt::Display| -> crate::Error { format!("string enum {package_path}.{type_name}: {err}").into() };

            let definition_path = format!("io.k8s.api.{}.{type_name}", package_path.replace('/', "."));
            if definitions.contains_key(&*definition_path) {
                return Err(error(&format_args!("conflicts with definition {definition_path}")));
            }

            let variants = get_variants(&packages, package_path, package, type_name).map_err(|err| error(&err))?;

            let mod_name = k8s_openapi_codegen_common::get_rust_ident(type_name);
            let file_name = dir.join(&*mod_name).with_extension("rs");
            let mut file = std::io::BufWriter::new(
                std::fs::OpenOptions::new().write(true).create_new(true).open(&file_name)
                .map_err(|err| error(&format_args!("could not create {}: {err}", file_name.display())))?);

            writeln!(file, "// Generated from Go type k8s.io/api/{package_path}.{type_name}")?;
            writeln!(file)?;
            writeln!(file, "crate::string_enum! {{")?;
            for line in k8s_openapi_codegen_common::get_comment_text(&string_type.doc, "") {
                writeln!(file, "    ///{line}")?;
            }
            if let Some(used_by) = used_by.get(&(package_path, type_name)) {
                if !string_type.doc.is_empty() {
                    writeln!(file, "    ///")?;
                }
                writeln!(file, "    /// Used by:")?;
                writeln!(file, "    ///")?;
                for field in used_by {
                    writeln!(file, "    /// - {field}")?;
                }
            }
            writeln!(file, "    pub enum {type_name} {{")?;
            for (i, Variant { name, value, doc }) in variants.iter().enumerate() {
                if i > 0 {
                    writeln!(file)?;
                }
                for line in k8s_openapi_codegen_common::get_comment_text(doc, "") {
                    writeln!(file, "        ///{line}")?;
                }
                writeln!(file, "        {name} = {value:?},")?;
            }
            writeln!(file, "    }}")?;
            writeln!(file, "}}")?;
            file.flush()?;

            writeln!(mod_rs)?;
            writeln!(mod_rs, "mod {mod_name};")?;
            writeln!(mod_rs, "pub use self::{mod_name}::{type_name};")?;

            num_generated += 1;
        }

        for (&type_name, type_alias) in &package.type_aliases {
            let Some((target_package_path, target_type_name)) = resolve_enum(&packages, type_alias.package.unwrap_or(package_path), None, type_alias.type_name) else {
                if type_alias.is_enum {
                    return Err(format!("string enum alias {package_path}.{type_name}: could not resolve {}.{}", type_alias.package.unwrap_or(package_path), type_alias.type_name).into());
                }
                continue;
            };

            let definition_path = format!("io.k8s.api.{}.{type_name}", package_path.replace('/', "."));
            if definitions.contains_key(&*definition_path) {
                return Err(format!("string enum alias {package_path}.{type_name}: conflicts with definition {definition_path}").into());
            }

            let target_type_path = format!("crate::api::{}::{target_type_name}", target_package_path.replace('/', "::"));
            writeln!(mod_rs)?;
            if target_type_name == type_name {
                writeln!(mod_rs, "pub use {target_type_path};")?;
            }
            else {
                writeln!(mod_rs, "pub use {target_type_path} as {type_name};")?;
            }
        }

        mod_rs.flush()?;
    }

    Ok(num_generated)
}

struct Variant<'a> {
    name: std::borrow::Cow<'a, str>,
    value: &'a str,
    doc: String,
}

fn get_variants<'a>(
    packages: &std::collections::BTreeMap<&str, Package<'a>>,
    package_path: &str,
    package: &Package<'a>,
    type_name: &str,
) -> Result<Vec<Variant<'a>>, crate::Error> {
    let consts: Vec<_> = package.consts.iter().filter(|c| c.type_name == Some(type_name)).collect();

    // Common prefix of the const names that ends at a word boundary, like `HostPath` in `HostPathDirectory`, `HostPathFile`, etc.
    // This is stripped from the const name of a variant whose value is not a valid variant name.
    let common_prefix_len = {
        let mut len = consts.iter().map(|c| c.name).reduce(|a, b| {
            let len = a.bytes().zip(b.bytes()).take_while(|(a, b)| a == b).count();
            &a[..len]
        }).map_or(0, str::len);
        while len > 0 && !(
            consts[0].name.as_bytes()[len - 1].is_ascii_lowercase() &&
            consts.iter().all(|c| c.name.as_bytes().get(len).is_some_and(u8::is_ascii_uppercase))
        ) {
            len -= 1;
        }
        len
    };

    let mut variants: Vec<Variant<'a>> = vec![];
    for c in consts {
        let value = resolve_const(packages, package_path, c, 0)?;
        if variants.iter().any(|variant| variant.value == value) {
            // Alias of an earlier const, like a deprecated name.
            continue;
        }

        let name =
            if is_valid_variant_name(value) { value.into() }
            else if is_valid_variant_name(&c.name[common_prefix_len..]) { c.name[common_prefix_len..].into() }
            else if is_valid_variant_name(c.name) { c.name.into() }
            else { return Err(format!("could not derive a variant name for const {} with value {value:?}", c.name).into()); };
        if variants.iter().any(|variant| variant.name == name) {
            return Err(format!("duplicate variant name {name} for const {}", c.name).into());
        }

        variants.push(Variant { name, value, doc: c.doc.clone() });
    }

    if variants.is_empty() {
        return Err("no values found".into());
    }

    Ok(variants)
}

fn resolve_const<'a>(
    packages: &std::collections::BTreeMap<&str, Package<'a>>,
    package_path: &str,
    c: &Const<'a>,
    depth: usize,
) -> Result<&'a str, crate::Error> {
    if depth > 8 {
        return Err(format!("const {} is too deeply nested", c.name).into());
    }

    match c.value {
        Expr::Literal(value) => Ok(value),
        Expr::Ident(alias, name) => {
            let package = &packages[package_path];
            let other_package_path = match alias {
                Some(alias) => *package.imports.get(alias).ok_or_else(|| format!("const {}: unknown import {alias}", c.name))?,
                None => package_path,
            };
            let other_package = packages.get(other_package_path).ok_or_else(|| format!("const {}: package {other_package_path} was not fetched", c.name))?;
            let other = other_package.consts.iter().find(|c| c.name == name).ok_or_else(|| format!("const {}: could not resolve {name}", c.name))?;
            resolve_const(packages, other_package_path, other, depth + 1)
        },
    }
}

fn is_valid_variant_name(s: &str) -> bool {
    s.starts_with(|c: char| c.is_ascii_uppercase()) && s.chars().all(|c| c.is_ascii_alphanumeric()) && s != "Self"
}

/// The parts of a Go `types.go` file that are relevant to string enums.
#[derive(Default)]
struct Package<'a> {
    /// Import alias => package path relative to `k8s.io/api`
    imports: std::collections::BTreeMap<&'a str, &'a str>,

    /// `type Foo string` declarations
    string_types: std::collections::BTreeMap<&'a str, StringType>,

    /// `type Foo = alias.Bar` declarations
    type_aliases: std::collections::BTreeMap<&'a str, TypeAlias<'a>>,

    /// `const` declarations whose values are strings or references to other consts, in declaration order
    consts: Vec<Const<'a>>,

    /// Struct fields
    fields: Vec<Field<'a>>,
}

impl Package<'_> {
    fn is_enum(&self, package_path: &str, type_name: &str) -> bool {
        self.string_types.get(type_name).is_some_and(|string_type|
            string_type.is_enum || EXTRA_ENUMS.contains(&(package_path, type_name)))
    }
}

/// Resolves the given type to the package and name of the string enum it refers to, following type aliases.
fn resolve_enum<'a>(
    packages: &std::collections::BTreeMap<&'a str, Package<'a>>,
    package_path: &'a str,
    type_package_alias: Option<&'a str>,
    type_name: &'a str,
) -> Option<(&'a str, &'a str)> {
    let mut package_path = match type_package_alias {
        Some(alias) => *packages.get(package_path)?.imports.get(alias)?,
        None => package_path,
    };
    let mut type_name = type_name;
    loop {
        let package = packages.get(package_path)?;
        if package.is_enum(package_path, type_name) {
            return Some((package_path, type_name));
        }
        let type_alias = package.type_aliases.get(type_name)?;
        (package_path, type_name) = (type_alias.package.unwrap_or(package_path), type_alias.type_name);
    }
}

struct TypeAlias<'a> {
    /// `None` if the aliased type is in the same package
    package: Option<&'a str>,
    type_name: &'a str,
    is_enum: bool,
}

struct StringType {
    doc: String,
    is_enum: bool,
}

struct Const<'a> {
    name: &'a str,
    type_name: Option<&'a str>,
    value: Expr<'a>,
    doc: String,
}

enum Expr<'a> {
    Literal(&'a str),
    Ident(Option<&'a str>, &'a str),
}

struct Field<'a> {
    struct_name: &'a str,
    json_name: &'a str,
    type_package: Option<&'a str>,
    type_name: &'a str,
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
                else if let Some(field) = parse_field(struct_name, trimmed) {
                    package.fields.push(field);
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
    let Some(package_path) = path.strip_prefix("k8s.io/api/") else { return };
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

/// Parses a struct field like ``Name *Type `json:"name,omitempty"` ``, where the type may also be a slice and may be qualified by an import alias.
fn parse_field<'a>(struct_name: &'a str, s: &'a str) -> Option<Field<'a>> {
    let mut parts = s.split_whitespace();
    let _name = parts.next()?;
    let ty = parts.next()?;
    let ty = ty.trim_start_matches("[]").trim_start_matches('*');
    let (type_package, type_name) = match ty.split_once('.') {
        Some((alias, type_name)) => (Some(alias), type_name),
        None => (None, ty),
    };
    if type_name.is_empty() || !type_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }

    let (_, tag) = s.split_once("`json:\"")?;
    let json_name = &tag[..tag.find([',', '"'])?];
    if json_name.is_empty() {
        return None;
    }

    Some(Field { struct_name, json_name, type_package, type_name })
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
