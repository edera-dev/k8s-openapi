// Generated from Go type k8s.io/api/core/v1.RestartPolicy

crate::string_enum! {
    /// RestartPolicy describes how the container should be restarted.
    /// Only one of the following restart policies may be specified.
    /// If none of the following policies is specified, the default one
    /// is RestartPolicyAlways.
    ///
    /// Used by:
    ///
    /// - [`PodSpec::restart_policy`](crate::api::core::v1::PodSpec::restart_policy)
    pub enum RestartPolicy {
        Always = "Always",

        OnFailure = "OnFailure",

        Never = "Never",
    }
}
