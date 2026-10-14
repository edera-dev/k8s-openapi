// Generated from Go type k8s.io/api/apps/v1.DeploymentStrategyType

crate::string_enum! {
    /// Used by:
    ///
    /// - [`DeploymentStrategy::type_`](crate::api::apps::v1::DeploymentStrategy::type_)
    pub enum DeploymentStrategyType {
        /// Kill all existing pods before creating new ones.
        Recreate = "Recreate",

        /// Replace the old ReplicaSets by new one using rolling update i.e gradually scale down the old ReplicaSets and scale up the new one.
        RollingUpdate = "RollingUpdate",
    }
}
