// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.Table

/// Table is a tabular representation of a set of API resources. The server transforms the
/// object into a set of preferred columns for quickly reviewing the objects.
///
/// This type is not part of the upstream OpenAPI spec. It is generated from the Go type k8s.io/apimachinery/pkg/apis/meta/v1.Table.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Table {
    /// columnDefinitions describes each column in the returned items array. The number of cells per row
    /// will always match the number of column definitions.
    pub column_definitions: std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::TableColumnDefinition>,

    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    pub metadata: crate::apimachinery::pkg::apis::meta::v1::ListMeta,

    /// rows is the list of items in the table.
    pub rows: std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::TableRow>,
}

impl crate::Resource for Table {
    const API_VERSION: &'static str = "meta.k8s.io/v1";
    const GROUP: &'static str = "meta.k8s.io";
    const KIND: &'static str = "Table";
    const VERSION: &'static str = "v1";
    const URL_PATH_SEGMENT: &'static str = "";
    type Scope = crate::SubResourceScope;
}

impl crate::Metadata for Table {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ListMeta;

    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut<Self as crate::Metadata>::Ty {
        &mut self.metadata
    }
}

impl crate::DeepMerge for Table {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.column_definitions, other.column_definitions);
        crate::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        crate::merge_strategies::list::atomic(&mut self.rows, other.rows);
    }
}

impl<'de> crate::serde::Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_column_definitions,
            Key_metadata,
            Key_rows,
            Other,
        }

        impl<'de> crate::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
                struct Visitor;

                impl crate::serde::de::Visitor<'_> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: crate::serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "columnDefinitions" => Field::Key_column_definitions,
                            "metadata" => Field::Key_metadata,
                            "rows" => Field::Key_rows,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = Table;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(<Self::Value as crate::Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_column_definitions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::TableColumnDefinition>> = None;
                let mut value_metadata: Option<crate::apimachinery::pkg::apis::meta::v1::ListMeta> = None;
                let mut value_rows: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::TableRow>> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as crate::Resource>::API_VERSION {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_api_version), &<Self::Value as crate::Resource>::API_VERSION));
                            }
                        },
                        Field::Key_kind => {
                            let value_kind: std::string::String = crate::serde::de::MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as crate::Resource>::KIND {
                                return Err(crate::serde::de::Error::invalid_value(crate::serde::de::Unexpected::Str(&value_kind), &<Self::Value as crate::Resource>::KIND));
                            }
                        },
                        Field::Key_column_definitions => value_column_definitions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rows => value_rows = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Table {
                    column_definitions: value_column_definitions.unwrap_or_default(),
                    metadata: value_metadata.unwrap_or_default(),
                    rows: value_rows.unwrap_or_default(),
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as crate::Resource>::KIND,
            &[
                "apiVersion",
                "kind",
                "columnDefinitions",
                "metadata",
                "rows",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for Table {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            <Self as crate::Resource>::KIND,
            5,
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", <Self as crate::Resource>::API_VERSION)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", <Self as crate::Resource>::KIND)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "columnDefinitions", &self.column_definitions)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "rows", &self.rows)?;
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for Table {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.apimachinery.pkg.apis.meta.v1.Table".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "Table is a tabular representation of a set of API resources. The server transforms the\nobject into a set of preferred columns for quickly reviewing the objects.\n\nThis type is not part of the upstream OpenAPI spec. It is generated from the Go type k8s.io/apimachinery/pkg/apis/meta/v1.Table.",
            "type": "object",
            "properties": {
                "apiVersion": {
                    "description": "APIVersion defines the versioned schema of this representation of an object.\nServers should convert recognized schemas to the latest internal value, and\nmay reject unrecognized values.\nMore info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources",
                    "type": "string",
                },
                "columnDefinitions": {
                    "description": "columnDefinitions describes each column in the returned items array. The number of cells per row\nwill always match the number of column definitions.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::TableColumnDefinition>()),
                },
                "kind": {
                    "description": "Kind is a string value representing the REST resource this object represents.\nServers may infer this from the endpoint the client submits requests to.\nCannot be updated.\nIn CamelCase.\nMore info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds",
                    "type": "string",
                },
                "metadata": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::ListMeta>();
                    schema_obj.ensure_object().insert("description".into(), "Standard list metadata.\nMore info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds".into());
                    schema_obj
                }),
                "rows": {
                    "description": "rows is the list of items in the table.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::TableRow>()),
                },
            },
            "required": [
                "columnDefinitions",
                "metadata",
                "rows",
            ],
        })
    }
}
