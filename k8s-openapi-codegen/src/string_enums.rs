//! Enums for string fields whose values are one of a fixed set, like phases, policies and modes.
//!
//! The upstream spec types these fields as plain strings and only lists their values in the field's description.
//! However the Go source marks such string types with a `// +enum` comment
//! and defines their values as constants, so the enums are generated from that instead.
//!
//! Each enum is emitted as an invocation of the `string_enum!` macro into the module corresponding to its Go package.

/// String types that are not marked `+enum` upstream because they are open to custom values,
/// but which are still useful to have enums for the well-known values.
const EXTRA_ENUMS: &[(&str, &str)] = &[
    ("api/core/v1", "ConditionStatus"),
    ("api/core/v1", "PodConditionType"),
    ("apimachinery/pkg/apis/meta/v1", "ConditionStatus"),
    ("apimachinery/pkg/apis/meta/v1", "IncludeObjectPolicy"),
    ("apimachinery/pkg/apis/meta/v1", "RowConditionType"),
];

pub(crate) fn generate(
    definitions: &std::collections::BTreeMap<crate::swagger20::DefinitionPath, crate::swagger20::Schema>,
    packages: &crate::go_source::Packages<'_>,
    out_dir: &std::path::Path,
) -> Result<usize, crate::Error> {
    use std::io::Write;

    for &(package_path, type_name) in EXTRA_ENUMS {
        if !packages.get(package_path).is_some_and(|package| package.string_types.contains_key(type_name)) {
            return Err(format!("extra string enum {package_path}.{type_name} not found").into());
        }
    }

    // The fields of each enum, for the "Used by" section of its docs.
    let mut used_by: std::collections::BTreeMap<(&str, &str), Vec<String>> = Default::default();
    for (&package_path, package) in packages {
        for &struct_name in &package.struct_names {
            let go_struct = &package.structs[struct_name];
            let definition_path = format!("{}.{struct_name}", crate::go_source::definition_namespace(package_path));
            let Some(definition) = definitions.get(&*definition_path) else { continue };
            let crate::swagger20::SchemaKind::Properties(properties) = &definition.kind else { continue };

            for field in &go_struct.fields {
                let Some(json_name) = field.json_name else { continue };
                let Some((type_package_alias, type_name)) = field.ty.element() else { continue };
                let Some(enum_key) = resolve_enum(packages, package_path, type_package_alias, type_name) else { continue };
                if !properties.contains_key(json_name) {
                    continue;
                }

                let rust_type_path = format!("{}::{struct_name}", crate::go_source::rust_module_path(package_path));
                let rust_field_name = k8s_openapi_codegen_common::get_rust_ident(json_name);
                used_by.entry(enum_key).or_default().push(
                    format!("[`{struct_name}::{rust_field_name}`]({rust_type_path}::{rust_field_name})"));
            }
        }
    }

    let mut num_generated = 0;

    for (&package_path, package) in packages {
        let dir = crate::go_source::rust_module_dir(out_dir, package_path);

        let mut mod_rs = std::io::BufWriter::new(std::fs::OpenOptions::new().append(true).open(dir.join("mod.rs"))?);

        for (&type_name, string_type) in &package.string_types {
            if !is_enum(package, package_path, type_name) {
                continue;
            }

            let error = |err: &dyn std::fmt::Display| -> crate::Error { format!("string enum {package_path}.{type_name}: {err}").into() };

            let definition_path = format!("{}.{type_name}", crate::go_source::definition_namespace(package_path));
            if definitions.contains_key(&*definition_path) {
                return Err(error(&format_args!("conflicts with definition {definition_path}")));
            }

            let variants = get_variants(packages, package_path, package, type_name).map_err(|err| error(&err))?;

            let mod_name = k8s_openapi_codegen_common::get_rust_ident(type_name);
            let file_name = dir.join(&*mod_name).with_extension("rs");
            let mut file = std::io::BufWriter::new(
                std::fs::OpenOptions::new().write(true).create_new(true).open(&file_name)
                .map_err(|err| error(&format_args!("could not create {}: {err}", file_name.display())))?);

            writeln!(file, "// Generated from Go type k8s.io/{package_path}.{type_name}")?;
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
            let Some((target_package_path, target_type_name)) = resolve_enum(packages, type_alias.package.unwrap_or(package_path), None, type_alias.type_name) else {
                if type_alias.is_enum {
                    return Err(format!("string enum alias {package_path}.{type_name}: could not resolve {}.{}", type_alias.package.unwrap_or(package_path), type_alias.type_name).into());
                }
                continue;
            };

            let definition_path = format!("{}.{type_name}", crate::go_source::definition_namespace(package_path));
            if definitions.contains_key(&*definition_path) {
                return Err(format!("string enum alias {package_path}.{type_name}: conflicts with definition {definition_path}").into());
            }

            let target_type_path = format!("{}::{target_type_name}", crate::go_source::rust_module_path(target_package_path));
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
    packages: &crate::go_source::Packages<'a>,
    package_path: &str,
    package: &crate::go_source::Package<'a>,
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
    packages: &crate::go_source::Packages<'a>,
    package_path: &str,
    c: &crate::go_source::Const<'a>,
    depth: usize,
) -> Result<&'a str, crate::Error> {
    if depth > 8 {
        return Err(format!("const {} is too deeply nested", c.name).into());
    }

    match c.value {
        crate::go_source::Expr::Literal(value) => Ok(value),
        crate::go_source::Expr::Ident(alias, name) => {
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

fn is_enum(package: &crate::go_source::Package<'_>, package_path: &str, type_name: &str) -> bool {
    package.string_types.get(type_name).is_some_and(|string_type|
        string_type.is_enum || EXTRA_ENUMS.contains(&(package_path, type_name)))
}

/// Resolves the given type to the package and name of the string enum it refers to, following type aliases.
fn resolve_enum<'a>(
    packages: &crate::go_source::Packages<'a>,
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
        if is_enum(package, package_path, type_name) {
            return Some((package_path, type_name));
        }
        let type_alias = package.type_aliases.get(type_name)?;
        (package_path, type_name) = (type_alias.package.unwrap_or(package_path), type_alias.type_name);
    }
}
