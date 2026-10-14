// Generated from definition io.k8s.apimachinery.pkg.apis.meta.v1.TableRow

/// TableRow is an individual row in a table.
///
/// This type is not part of the upstream OpenAPI spec. It is generated from the Go type k8s.io/apimachinery/pkg/apis/meta/v1.TableRow.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TableRow {
    /// cells will be as wide as the column definitions array and may contain strings, numbers (float64 or
    /// int64), booleans, simple maps, lists, or null. See the type field of the column definition for a
    /// more detailed description.
    pub cells: std::vec::Vec<crate::serde_json::Value>,

    /// conditions describe additional status of a row that are relevant for a human user. These conditions
    /// apply to the row, not to the object, and will be specific to table output. The only defined
    /// condition type is 'Completed', for a row that indicates a resource that has run to completion and
    /// can be given less visual priority.
    pub conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::TableRowCondition>>,

    /// This field contains the requested additional information about each object based on the includeObject
    /// policy when requesting the Table. If "None", this field is empty, if "Object" this will be the
    /// default serialization of the object for the current API version, and if "Metadata" (the default) will
    /// contain the object metadata. Check the returned kind and apiVersion of the object before parsing.
    /// The media type of the object will always match the enclosing list - if this as a JSON table, these
    /// will be JSON encoded objects.
    pub object: Option<crate::apimachinery::pkg::runtime::RawExtension>,
}

impl crate::DeepMerge for TableRow {
    fn merge_from(&mut self, other: Self) {
        crate::merge_strategies::list::atomic(&mut self.cells, other.cells);
        crate::merge_strategies::list::atomic(&mut self.conditions, other.conditions);
        crate::DeepMerge::merge_from(&mut self.object, other.object);
    }
}

impl<'de> crate::serde::Deserialize<'de> for TableRow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_cells,
            Key_conditions,
            Key_object,
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
                            "cells" => Field::Key_cells,
                            "conditions" => Field::Key_conditions,
                            "object" => Field::Key_object,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> crate::serde::de::Visitor<'de> for Visitor {
            type Value = TableRow;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("TableRow")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: crate::serde::de::MapAccess<'de> {
                let mut value_cells: Option<std::vec::Vec<crate::serde_json::Value>> = None;
                let mut value_conditions: Option<std::vec::Vec<crate::apimachinery::pkg::apis::meta::v1::TableRowCondition>> = None;
                let mut value_object: Option<crate::apimachinery::pkg::runtime::RawExtension> = None;

                while let Some(key) = crate::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_cells => value_cells = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_conditions => value_conditions = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_object => value_object = crate::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: crate::serde::de::IgnoredAny = crate::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(TableRow {
                    cells: value_cells.unwrap_or_default(),
                    conditions: value_conditions,
                    object: value_object,
                })
            }
        }

        deserializer.deserialize_struct(
            "TableRow",
            &[
                "cells",
                "conditions",
                "object",
            ],
            Visitor,
        )
    }
}

impl crate::serde::Serialize for TableRow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "TableRow",
            1 +
            self.conditions.as_ref().map_or(0, |_| 1) +
            self.object.as_ref().map_or(0, |_| 1),
        )?;
        crate::serde::ser::SerializeStruct::serialize_field(&mut state, "cells", &self.cells)?;
        if let Some(value) = &self.conditions {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "conditions", value)?;
        }
        if let Some(value) = &self.object {
            crate::serde::ser::SerializeStruct::serialize_field(&mut state, "object", value)?;
        }
        crate::serde::ser::SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl crate::schemars::JsonSchema for TableRow {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "io.k8s.apimachinery.pkg.apis.meta.v1.TableRow".into()
    }

    fn json_schema(__gen: &mut crate::schemars::SchemaGenerator) -> crate::schemars::Schema {
        crate::schemars::json_schema!({
            "description": "TableRow is an individual row in a table.\n\nThis type is not part of the upstream OpenAPI spec. It is generated from the Go type k8s.io/apimachinery/pkg/apis/meta/v1.TableRow.",
            "type": "object",
            "properties": {
                "cells": {
                    "description": "cells will be as wide as the column definitions array and may contain strings, numbers (float64 or\nint64), booleans, simple maps, lists, or null. See the type field of the column definition for a\nmore detailed description.",
                    "type": "array",
                    "items": {
                        "type": "object",
                    },
                },
                "conditions": {
                    "description": "conditions describe additional status of a row that are relevant for a human user. These conditions\napply to the row, not to the object, and will be specific to table output. The only defined\ncondition type is 'Completed', for a row that indicates a resource that has run to completion and\ncan be given less visual priority.",
                    "type": "array",
                    "items": (__gen.subschema_for::<crate::apimachinery::pkg::apis::meta::v1::TableRowCondition>()),
                },
                "object": ({
                    let mut schema_obj = __gen.subschema_for::<crate::apimachinery::pkg::runtime::RawExtension>();
                    schema_obj.ensure_object().insert("description".into(), "This field contains the requested additional information about each object based on the includeObject\npolicy when requesting the Table. If \"None\", this field is empty, if \"Object\" this will be the\ndefault serialization of the object for the current API version, and if \"Metadata\" (the default) will\ncontain the object metadata. Check the returned kind and apiVersion of the object before parsing.\nThe media type of the object will always match the enclosing list - if this as a JSON table, these\nwill be JSON encoded objects.".into());
                    schema_obj
                }),
            },
            "required": [
                "cells",
            ],
        })
    }
}
