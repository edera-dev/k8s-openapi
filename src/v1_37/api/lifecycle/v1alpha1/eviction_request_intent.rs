// Generated from Go type k8s.io/api/lifecycle/v1alpha1.EvictionRequestIntent

crate::string_enum! {
    /// EvictionRequestIntent specifies a requester intent.
    ///
    /// Used by:
    ///
    /// - [`EvictionRequestSpec::intent`](crate::api::lifecycle::v1alpha1::EvictionRequestSpec::intent)
    pub enum EvictionRequestIntent {
        /// EvictionRequestIntentEviction means that the requester is interested in the eviction of the target.
        Eviction = "Eviction",

        /// EvictionRequestIntentWithdrawn means that the requester is no longer interested in the eviction of the target.
        /// If all requesters' intents are withdrawn for a common target, the eviction will be canceled.
        /// Cancellation consequences:
        /// - Inactive responders will never run.
        /// - Active responders are expected to cancel the eviction.
        /// - Completed or Interrupted responders should not take any action.
        Withdrawn = "Withdrawn",
    }
}
