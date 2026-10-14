// Generated from Go type k8s.io/api/core/v1.PreemptionPolicy

crate::string_enum! {
    /// PreemptionPolicy describes a policy for if/when to preempt a pod.
    ///
    /// Used by:
    ///
    /// - [`PodSpec::preemption_policy`](crate::api::core::v1::PodSpec::preemption_policy)
    /// - [`PriorityClass::preemption_policy`](crate::api::scheduling::v1::PriorityClass::preemption_policy)
    pub enum PreemptionPolicy {
        /// PreemptLowerPriority means that pod can preempt other pods with lower priority.
        PreemptLowerPriority = "PreemptLowerPriority",

        /// PreemptNever means that pod never preempts other pods with lower priority.
        Never = "Never",
    }
}
