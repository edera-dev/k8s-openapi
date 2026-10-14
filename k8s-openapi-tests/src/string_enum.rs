use k8s_openapi::api::core::v1 as api;

#[test]
fn round_trip() {
    for &phase in api::PodPhase::VARIANTS {
        assert_eq!(api::PodPhase::parse(phase.as_str()), Some(phase));
        assert_eq!(phase.as_str().parse::<api::PodPhase>(), Ok(phase));
        assert_eq!(phase.to_string(), phase.as_str());
    }

    assert_eq!(api::PodPhase::parse("Bogus"), None);
    let err = "Bogus".parse::<api::PodPhase>().unwrap_err();
    assert_eq!(err.type_name(), "PodPhase");
    assert_eq!(err.value(), "Bogus");
    assert_eq!(err.to_string(), r#"unknown PodPhase value "Bogus""#);
}

#[test]
fn compare_with_strings() {
    let condition = api::PodCondition { type_: "Ready".to_owned(), status: "True".to_owned(), ..Default::default() };
    assert!(condition.type_ == api::PodConditionType::Ready);
    assert!(api::PodConditionType::Ready == condition.type_);
    assert!(condition.type_ != api::PodConditionType::PodScheduled);
    assert!(condition.status == api::ConditionStatus::True);
    assert!(*"Ready" == api::PodConditionType::Ready);
    assert!(api::PodConditionType::Ready == "Ready");

    let status = api::PodStatus { phase: Some("Running".to_owned()), ..Default::default() };
    let phase = status.phase.as_deref().and_then(api::PodPhase::parse).unwrap_or(api::PodPhase::Unknown);
    assert_eq!(phase, api::PodPhase::Running);

    let status = api::PodStatus::default();
    let phase = status.phase.as_deref().and_then(api::PodPhase::parse).unwrap_or(api::PodPhase::Unknown);
    assert_eq!(phase, api::PodPhase::Unknown);
}

#[test]
fn serde() {
    let phase: api::PodPhase = serde_json::from_str(r#""Succeeded""#).unwrap();
    assert_eq!(phase, api::PodPhase::Succeeded);
    assert_eq!(serde_json::to_string(&phase).unwrap(), r#""Succeeded""#);

    let err = serde_json::from_str::<api::PodPhase>(r#""Bogus""#).unwrap_err();
    assert!(err.to_string().contains("unknown variant `Bogus`"), "{err}");
}

#[test]
fn generated_from_go_source() {
    use k8s_openapi::api::{admissionregistration, batch, certificates};

    // Values that are not valid variant names use the Go const name instead.
    assert_eq!(api::HostPathType::Unset.as_str(), "");
    assert_eq!(certificates::v1::KeyUsage::parse("digital signature"), Some(certificates::v1::KeyUsage::DigitalSignature));
    assert_eq!(admissionregistration::v1::OperationType::OperationAll.as_str(), "*");

    // Go type aliases are re-exports of the same enum.
    let _: admissionregistration::v1::ScopeType = admissionregistration::v1beta1::ScopeType::Cluster;
    let _: api::IPFamilyPolicy = api::IPFamilyPolicyType::SingleStack;

    // Enums from packages other than core/v1, compared against their fields.
    let spec = batch::v1::JobSpec { completion_mode: Some("Indexed".to_owned()), ..Default::default() };
    assert!(spec.completion_mode.as_deref() == Some(batch::v1::CompletionMode::Indexed.as_str()));

    let pv = api::PersistentVolumeSpec { access_modes: Some(vec!["ReadWriteOnce".to_owned()]), ..Default::default() };
    assert!(pv.access_modes.unwrap().iter().any(|mode| *mode == api::PersistentVolumeAccessMode::ReadWriteOnce));
}
