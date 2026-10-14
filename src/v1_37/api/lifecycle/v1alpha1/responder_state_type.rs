// Generated from Go type k8s.io/api/lifecycle/v1alpha1.ResponderStateType

crate::string_enum! {
    /// ResponderStateType specifies a state that is assigned by the evictionrequest-controller.
    ///
    /// Used by:
    ///
    /// - [`TargetResponder::state`](crate::api::lifecycle::v1alpha1::TargetResponder::state)
    pub enum ResponderStateType {
        /// ResponderStateInactive means that the responder should not yet process this eviction request.
        Inactive = "Inactive",

        /// ResponderStateActive means that the responder is either running or expected to start soon.
        /// Also, startTime has been set in the ResponderStatus by the evictionrequest-controller.
        ///
        /// An active responder should currently interact with the eviction process by updating
        /// .status.responders, where .name is the active responder name. ResponderStatus fields
        /// should be periodically updated to indicate the progress or completion of the eviction process.
        /// If .status.responders\[\].heartbeatTime field is not updated within the heartbeat deadline defined
        /// by the Eviction API (currently 20 minutes), the eviction is passed over to the next responder
        /// with a lower priority. Only one responder can be active at a time.
        Active = "Active",

        /// ResponderStateInterrupted means that the responder has failed to start or failed to update
        /// heartbeatTime in ResponderStatus in a timely manner.
        Interrupted = "Interrupted",

        /// ResponderStateCanceled means that the responder has been canceled. In other words, there
        /// is no EvictionRequest with the same target and Eviction intent in .spec.intent.
        Canceled = "Canceled",

        /// ResponderStateCompleted means that the responder has successfully completed and set completionTime
        /// in ResponderStatus.
        Completed = "Completed",
    }
}
