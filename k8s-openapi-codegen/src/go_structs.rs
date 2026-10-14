//! Definitions for types that exist in the upstream Go source but not in the OpenAPI spec, like `meta/v1.Table`.
//!
//! The API server returns these types (for example when a client requests `Accept: application/json;as=Table;g=meta.k8s.io;v=v1`),
//! but they are not part of the spec because they are not served by any OpenAPI-described operation.
//! Their Go structs are converted into spec definitions so that they go through the same codegen as every other type.

/// Go structs to add to the spec. Structs in the same package that they reference are added too, if they are also missing from the spec.
const GO_STRUCTS: &[GoStruct] = &[
    GoStruct { package_path: "apimachinery/pkg/apis/meta/v1", name: "PartialObjectMetadata", group_version: None },
    GoStruct { package_path: "apimachinery/pkg/apis/meta/v1", name: "Table", group_version: Some(("meta.k8s.io", "v1")) },
];

struct GoStruct {
    package_path: &'static str,
    name: &'static str,

    /// The group and version to make the type a `Resource` with, using the struct name as the kind.
    /// The resource's URL path segment and scope must also be special-cased in `k8s_openapi_codegen_common::run`
    /// since they cannot be inferred from the spec's operations.
    group_version: Option<(&'static str, &'static str)>,
}

/// Adds definitions for [`GO_STRUCTS`] to the spec. Returns the number of definitions added.
pub(crate) fn add_definitions(
    spec: &mut crate::swagger20::Spec,
    packages: &crate::go_source::Packages<'_>,
) -> Result<usize, crate::Error> {
    let mut pending: std::collections::VecDeque<(&str, &str)> = GO_STRUCTS.iter().map(|go_struct| (go_struct.package_path, go_struct.name)).collect();
    let mut num_added = 0;

    while let Some((package_path, struct_name)) = pending.pop_front() {
        let definition_path = crate::swagger20::DefinitionPath(format!("{}.{struct_name}", crate::go_source::definition_namespace(package_path)));
        if spec.definitions.contains_key(&definition_path) {
            if GO_STRUCTS.iter().any(|go_struct| go_struct.package_path == package_path && go_struct.name == struct_name) {
                return Err(format!("Go struct {package_path}.{struct_name} is already in the spec and does not need to be added").into());
            }
            continue;
        }

        let error = |err: &dyn std::fmt::Display| -> crate::Error { format!("Go struct {package_path}.{struct_name}: {err}").into() };

        let go_struct =
            packages.get(package_path).and_then(|package| package.structs.get(struct_name))
            .ok_or_else(|| error(&"not found"))?;

        let mut properties = Default::default();
        add_properties(&mut properties, &spec.definitions, packages, package_path, go_struct, &mut pending).map_err(|err| error(&err))?;

        // Same as the `resource_metadata_not_optional` fixup, which has already run on the rest of the spec.
        let has_api_version_and_kind = properties.contains_key("apiVersion") && properties.contains_key("kind");
        if has_api_version_and_kind && let Some((_, required)) = properties.get_mut("metadata") {
            *required = true;
        }

        let kubernetes_group_kind_versions =
            match GO_STRUCTS.iter().find(|go_struct| go_struct.package_path == package_path && go_struct.name == struct_name) {
                Some(GoStruct { group_version: Some((group, version)), .. }) => {
                    if !has_api_version_and_kind {
                        return Err(error(&"has a group and version but does not have apiVersion and kind fields"));
                    }
                    vec![crate::swagger20::KubernetesGroupKindVersion {
                        group: (*group).to_owned(),
                        kind: struct_name.to_owned(),
                        version: (*version).to_owned(),
                    }]
                },
                _ => vec![],
            };

        let description = format!(
            "{}\n\nThis type is not part of the upstream OpenAPI spec. It is generated from the Go type k8s.io/{package_path}.{struct_name}.",
            go_struct.doc,
        );

        spec.definitions.insert(definition_path, crate::swagger20::Schema {
            description: Some(description),
            kind: crate::swagger20::SchemaKind::Properties(properties),
            kubernetes_group_kind_versions,
            list_kind: None,
            merge_type: crate::swagger20::MergeType::Default,
            impl_deep_merge: true,
        });

        num_added += 1;
    }

    Ok(num_added)
}

fn add_properties<'a>(
    properties: &mut std::collections::BTreeMap<crate::swagger20::PropertyName, (crate::swagger20::Schema, bool)>,
    definitions: &std::collections::BTreeMap<crate::swagger20::DefinitionPath, crate::swagger20::Schema>,
    packages: &crate::go_source::Packages<'a>,
    package_path: &'a str,
    go_struct: &crate::go_source::Struct<'a>,
    pending: &mut std::collections::VecDeque<(&'a str, &'a str)>,
) -> Result<(), crate::Error> {
    for field in &go_struct.fields {
        let Some(json_name) = field.json_name else {
            // Embedded struct whose fields are inlined, like `TypeMeta`
            let Some((alias, type_name)) = field.ty.element() else {
                return Err(format!("embedded field of type {:?} is not a named type", field.ty).into());
            };
            let embedded_package_path = resolve_package(packages, package_path, alias)?;
            let embedded_struct =
                packages.get(embedded_package_path).and_then(|package| package.structs.get(type_name))
                .ok_or_else(|| format!("embedded struct {embedded_package_path}.{type_name} not found"))?;
            add_properties(properties, definitions, packages, embedded_package_path, embedded_struct, pending)?;
            continue;
        };

        let mut schema = get_schema(definitions, packages, package_path, &field.ty, &field.markers, pending)
            .map_err(|err| format!("field {json_name}: {err}"))?;
        schema.description = (!field.doc.is_empty()).then(|| field.doc.clone());

        let required = !field.omitempty && !field.markers.contains(&"optional");

        if properties.insert(crate::swagger20::PropertyName(json_name.to_owned()), (schema, required)).is_some() {
            return Err(format!("duplicate field {json_name}").into());
        }
    }

    Ok(())
}

fn get_schema<'a>(
    definitions: &std::collections::BTreeMap<crate::swagger20::DefinitionPath, crate::swagger20::Schema>,
    packages: &crate::go_source::Packages<'a>,
    package_path: &'a str,
    ty: &crate::go_source::Type<'a>,
    markers: &[&str],
    pending: &mut std::collections::VecDeque<(&'a str, &'a str)>,
) -> Result<crate::swagger20::Schema, crate::Error> {
    use crate::go_source::Type as GoType;
    use crate::swagger20::{IntegerFormat, MergeType, NumberFormat, SchemaKind, StringFormat, Type};

    let (kind, merge_type) = match ty {
        GoType::Pointer(ty) => return get_schema(definitions, packages, package_path, ty, markers, pending),

        GoType::Slice(item) if matches!(**item, GoType::Named(None, "byte")) =>
            (SchemaKind::Ty(Type::String { format: Some(StringFormat::Byte) }), MergeType::Default),

        GoType::Slice(item) => {
            let item = get_schema(definitions, packages, package_path, item, &[], pending)?;
            let strategy = match marker_value(markers, "listType=") {
                None | Some("atomic") => crate::swagger20::KubernetesListType::Atomic,
                Some("map") => crate::swagger20::KubernetesListType::Map,
                Some("set") => crate::swagger20::KubernetesListType::Set,
                Some(list_type) => return Err(format!("unknown listType {list_type}").into()),
            };
            let keys = markers.iter().filter_map(|marker| marker.strip_prefix("listMapKey=")).map(ToOwned::to_owned).collect();
            let merge_type = MergeType::List { strategy, keys, item_merge_type: Box::new(item.merge_type.clone()) };
            (SchemaKind::Ty(Type::Array { items: Box::new(item) }), merge_type)
        },

        GoType::Map(key, value) => {
            if !matches!(**key, GoType::Named(None, "string")) {
                return Err(format!("map key type {key:?} is not string").into());
            }
            let value = get_schema(definitions, packages, package_path, value, &[], pending)?;
            let strategy = match marker_value(markers, "mapType=") {
                None | Some("granular") => crate::swagger20::KubernetesMapType::Granular,
                Some("atomic") => crate::swagger20::KubernetesMapType::Atomic,
                Some(map_type) => return Err(format!("unknown mapType {map_type}").into()),
            };
            let merge_type = MergeType::Map { strategy, value_merge_type: Box::new(value.merge_type.clone()) };
            (SchemaKind::Ty(Type::Object { additional_properties: Box::new(value) }), merge_type)
        },

        GoType::Interface => (SchemaKind::Ty(Type::Any), MergeType::Default),

        GoType::Named(None, "string") => (SchemaKind::Ty(Type::String { format: None }), MergeType::Default),
        GoType::Named(None, "bool") => (SchemaKind::Ty(Type::Boolean), MergeType::Default),
        GoType::Named(None, "int32") => (SchemaKind::Ty(Type::Integer { format: IntegerFormat::Int32 }), MergeType::Default),
        GoType::Named(None, "int" | "int64") => (SchemaKind::Ty(Type::Integer { format: IntegerFormat::Int64 }), MergeType::Default),
        GoType::Named(None, "float64") => (SchemaKind::Ty(Type::Number { format: NumberFormat::Double }), MergeType::Default),

        &GoType::Named(alias, type_name) => {
            let type_package_path = resolve_package(packages, package_path, alias)?;
            let definition_path = format!("{}.{type_name}", crate::go_source::definition_namespace(type_package_path));
            let type_package = packages.get(type_package_path);

            let kind =
                if definitions.contains_key(&*definition_path) {
                    SchemaKind::Ref(crate::swagger20::RefPath { path: definition_path, can_be_default: None })
                }
                else if type_package.is_some_and(|package| package.string_types.contains_key(type_name) || package.type_aliases.contains_key(type_name)) {
                    // String types like enums are represented as plain strings, like the rest of the spec does.
                    SchemaKind::Ty(Type::String { format: None })
                }
                else if type_package.is_some_and(|package| package.structs.contains_key(type_name)) {
                    pending.push_back((type_package_path, type_name));
                    SchemaKind::Ref(crate::swagger20::RefPath { path: definition_path, can_be_default: None })
                }
                else {
                    return Err(format!("type {type_package_path}.{type_name} is neither in the spec nor in the fetched Go source").into());
                };
            (kind, MergeType::Default)
        },
    };

    Ok(crate::swagger20::Schema {
        description: None,
        kind,
        kubernetes_group_kind_versions: vec![],
        list_kind: None,
        merge_type,
        impl_deep_merge: true,
    })
}

fn resolve_package<'a>(packages: &crate::go_source::Packages<'a>, package_path: &'a str, alias: Option<&'a str>) -> Result<&'a str, crate::Error> {
    match alias {
        Some(alias) =>
            packages.get(package_path).and_then(|package| package.imports.get(alias)).copied()
            .ok_or_else(|| format!("unknown import {alias} in package {package_path}").into()),
        None => Ok(package_path),
    }
}

fn marker_value<'a>(markers: &[&'a str], prefix: &str) -> Option<&'a str> {
    markers.iter().find_map(|marker| marker.strip_prefix(prefix))
}
