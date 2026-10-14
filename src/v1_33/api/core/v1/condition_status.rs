// Generated from Go type k8s.io/api/core/v1.ConditionStatus

crate::string_enum! {
    /// Used by:
    ///
    /// - [`StatefulSetCondition::status`](crate::api::apps::v1::StatefulSetCondition::status)
    /// - [`DeploymentCondition::status`](crate::api::apps::v1::DeploymentCondition::status)
    /// - [`DaemonSetCondition::status`](crate::api::apps::v1::DaemonSetCondition::status)
    /// - [`ReplicaSetCondition::status`](crate::api::apps::v1::ReplicaSetCondition::status)
    /// - [`HorizontalPodAutoscalerCondition::status`](crate::api::autoscaling::v2::HorizontalPodAutoscalerCondition::status)
    /// - [`PodFailurePolicyOnPodConditionsPattern::status`](crate::api::batch::v1::PodFailurePolicyOnPodConditionsPattern::status)
    /// - [`JobCondition::status`](crate::api::batch::v1::JobCondition::status)
    /// - [`CertificateSigningRequestCondition::status`](crate::api::certificates::v1::CertificateSigningRequestCondition::status)
    /// - [`PersistentVolumeClaimCondition::status`](crate::api::core::v1::PersistentVolumeClaimCondition::status)
    /// - [`PodCondition::status`](crate::api::core::v1::PodCondition::status)
    /// - [`ReplicationControllerCondition::status`](crate::api::core::v1::ReplicationControllerCondition::status)
    /// - [`NodeCondition::status`](crate::api::core::v1::NodeCondition::status)
    /// - [`NamespaceCondition::status`](crate::api::core::v1::NamespaceCondition::status)
    /// - [`ComponentCondition::status`](crate::api::core::v1::ComponentCondition::status)
    /// - [`MigrationCondition::status`](crate::api::storagemigration::v1alpha1::MigrationCondition::status)
    pub enum ConditionStatus {
        True = "True",

        False = "False",

        Unknown = "Unknown",
    }
}
