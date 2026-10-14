// Generated from Go type k8s.io/api/lifecycle/v1alpha1.RequesterIntent

crate::string_enum! {
    /// RequesterIntent specifies a requester intent.
    ///
    /// Used by:
    ///
    /// - [`Requester::intent`](crate::api::lifecycle::v1alpha1::Requester::intent)
    pub enum RequesterIntent {
        /// RequesterIntentEviction means that the requester is interested in the eviction of the target.
        Eviction = "Eviction",

        /// RequesterIntentWithdrawn means that the requester is no longer interested in the eviction of the target.
        /// If all requesters' intents are withdrawn, the eviction will be canceled.
        /// Cancellation consequences:
        /// - Inactive responders will never run.
        /// - Active responders are expected to cancel the eviction.
        /// - Completed or Interrupted responders should not take any action.
        Withdrawn = "Withdrawn",
    }
}
