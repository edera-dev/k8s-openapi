use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

// The shape of the response to `GET /api/v1/namespaces/default/pods` with `Accept: application/json;as=Table;g=meta.k8s.io;v=v1`
const TABLE: &str = r#"{
    "kind": "Table",
    "apiVersion": "meta.k8s.io/v1",
    "metadata": { "resourceVersion": "4321" },
    "columnDefinitions": [
        { "name": "Name", "type": "string", "format": "name", "description": "Name must be unique within a namespace.", "priority": 0 },
        { "name": "Ready", "type": "string", "format": "", "description": "The aggregate readiness state of this pod for accepting traffic.", "priority": 0 },
        { "name": "Restarts", "type": "string", "format": "", "description": "The number of times the containers in this pod have been restarted.", "priority": 0 },
        { "name": "IP", "type": "string", "format": "", "description": "IP address allocated to the pod.", "priority": 1 }
    ],
    "rows": [
        {
            "cells": ["web-0", "1/1", 0, "10.0.0.5"],
            "object": {
                "kind": "PartialObjectMetadata",
                "apiVersion": "meta.k8s.io/v1",
                "metadata": { "name": "web-0", "namespace": "default", "resourceVersion": "4320" }
            }
        },
        {
            "cells": ["job-abc", "0/1", 2, null],
            "conditions": [{ "type": "Completed", "status": "True" }],
            "object": {
                "kind": "PartialObjectMetadata",
                "apiVersion": "meta.k8s.io/v1",
                "metadata": { "name": "job-abc", "namespace": "default" }
            }
        }
    ]
}"#;

#[test]
fn deserialize() {
    let table: meta::Table = serde_json::from_str(TABLE).unwrap();

    assert_eq!(table.metadata.resource_version.as_deref(), Some("4321"));

    let column_names: Vec<_> = table.column_definitions.iter().map(|column| &*column.name).collect();
    assert_eq!(column_names, ["Name", "Ready", "Restarts", "IP"]);
    assert_eq!(table.column_definitions[0].format, "name");
    assert_eq!(table.column_definitions[3].priority, 1);

    assert_eq!(table.rows.len(), 2);
    for row in &table.rows {
        assert_eq!(row.cells.len(), table.column_definitions.len());
    }
    assert_eq!(table.rows[0].cells[0], serde_json::json!("web-0"));
    assert_eq!(table.rows[1].cells[2], serde_json::json!(2));
    assert!(table.rows[1].cells[3].is_null());

    // The default includeObject policy returns each row's metadata as a PartialObjectMetadata.
    let object = table.rows[0].object.as_ref().unwrap();
    let object: meta::PartialObjectMetadata = serde_json::from_value(object.0.clone()).unwrap();
    assert_eq!(object.metadata.name.as_deref(), Some("web-0"));

    assert!(table.rows[0].conditions.is_none());
    let condition = &table.rows[1].conditions.as_ref().unwrap()[0];
    assert!(condition.type_ == meta::RowConditionType::Completed);
    assert!(condition.status == meta::ConditionStatus::True);
}

#[test]
fn round_trip() {
    let table: meta::Table = serde_json::from_str(TABLE).unwrap();
    let serialized = serde_json::to_value(&table).unwrap();
    let expected: serde_json::Value = serde_json::from_str(TABLE).unwrap();
    assert_eq!(serialized, expected);
}

#[test]
fn include_object_policy() {
    // The values of the `includeObject` query parameter.
    let values: Vec<_> = meta::IncludeObjectPolicy::VARIANTS.iter().map(|policy| policy.as_str()).collect();
    assert_eq!(values, ["None", "Metadata", "Object"]);
}

#[test]
fn resource() {
    use k8s_openapi::{Metadata, Resource};

    assert_eq!(meta::Table::API_VERSION, "meta.k8s.io/v1");
    assert_eq!(meta::Table::GROUP, "meta.k8s.io");
    assert_eq!(meta::Table::KIND, "Table");
    assert_eq!(meta::Table::VERSION, "v1");
    assert_eq!(k8s_openapi::kind(&meta::Table::default()), "Table");

    let table: meta::Table = serde_json::from_str(TABLE).unwrap();
    assert_eq!(table.metadata().resource_version.as_deref(), Some("4321"));

    // apiVersion and kind are emitted on serialization and checked on deserialization.
    let serialized = serde_json::to_value(meta::Table::default()).unwrap();
    assert_eq!(serialized["apiVersion"], "meta.k8s.io/v1");
    assert_eq!(serialized["kind"], "Table");

    let wrong_version = TABLE.replace("\"apiVersion\": \"meta.k8s.io/v1\",\n    \"metadata\"", "\"apiVersion\": \"meta.k8s.io/v1beta1\",\n    \"metadata\"");
    assert_ne!(wrong_version, TABLE);
    assert!(serde_json::from_str::<meta::Table>(&wrong_version).is_err());

    let wrong_kind = TABLE.replacen("\"kind\": \"Table\"", "\"kind\": \"PodList\"", 1);
    assert!(serde_json::from_str::<meta::Table>(&wrong_kind).is_err());
}
