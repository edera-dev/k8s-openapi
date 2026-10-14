// Generated from Go type k8s.io/api/scheduling/v1beta1.PreemptionPolicy

crate::string_enum! {
    /// PreemptionPolicy describes a policy for if/when to preempt a pod.
    ///
    /// Used by:
    ///
    /// - [`PodGroupTemplate::preemption_policy`](crate::api::scheduling::v1beta1::PodGroupTemplate::preemption_policy)
    /// - [`CompositePodGroupTemplate::preemption_policy`](crate::api::scheduling::v1beta1::CompositePodGroupTemplate::preemption_policy)
    /// - [`PodGroupSpec::preemption_policy`](crate::api::scheduling::v1beta1::PodGroupSpec::preemption_policy)
    pub enum PreemptionPolicy {
        /// PreemptLowerPriority means that pod can preempt other pods with lower priority.
        PreemptLowerPriority = "PreemptLowerPriority",

        /// PreemptNever means that pod never preempts other pods with lower priority.
        Never = "Never",
    }
}
