// Generated from Go type k8s.io/api/batch/v1.PodReplacementPolicy

crate::string_enum! {
    /// PodReplacementPolicy specifies the policy for creating pod replacements.
    ///
    /// Used by:
    ///
    /// - [`JobSpec::pod_replacement_policy`](crate::api::batch::v1::JobSpec::pod_replacement_policy)
    pub enum PodReplacementPolicy {
        /// TerminatingOrFailed means that we recreate pods
        /// when they are terminating (has a metadata.deletionTimestamp) or failed.
        TerminatingOrFailed = "TerminatingOrFailed",

        /// Failed means to wait until a previously created Pod is fully terminated (has phase
        /// Failed or Succeeded) before creating a replacement Pod.
        Failed = "Failed",
    }
}
