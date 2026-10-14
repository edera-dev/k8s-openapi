// Generated from Go type k8s.io/api/batch/v1.PodFailurePolicyAction

crate::string_enum! {
    /// PodFailurePolicyAction specifies how a Pod failure is handled.
    ///
    /// Used by:
    ///
    /// - [`PodFailurePolicyRule::action`](crate::api::batch::v1::PodFailurePolicyRule::action)
    pub enum PodFailurePolicyAction {
        /// This is an action which might be taken on a pod failure - mark the
        /// pod's job as Failed and terminate all running pods.
        FailJob = "FailJob",

        /// This is an action which might be taken on a pod failure - mark the
        /// Job's index as failed to avoid restarts within this index. This action
        /// can only be used when backoffLimitPerIndex is set.
        FailIndex = "FailIndex",

        /// This is an action which might be taken on a pod failure - the counter towards
        /// .backoffLimit, represented by the job's .status.failed field, is not
        /// incremented and a replacement pod is created.
        Ignore = "Ignore",

        /// This is an action which might be taken on a pod failure - the pod failure
        /// is handled in the default way - the counter towards .backoffLimit,
        /// represented by the job's .status.failed field, is incremented.
        Count = "Count",
    }
}
