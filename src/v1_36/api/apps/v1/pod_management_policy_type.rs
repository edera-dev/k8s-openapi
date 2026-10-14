// Generated from Go type k8s.io/api/apps/v1.PodManagementPolicyType

crate::string_enum! {
    /// PodManagementPolicyType defines the policy for creating pods under a stateful set.
    ///
    /// Used by:
    ///
    /// - [`StatefulSetSpec::pod_management_policy`](crate::api::apps::v1::StatefulSetSpec::pod_management_policy)
    pub enum PodManagementPolicyType {
        /// OrderedReadyPodManagement will create pods in strictly increasing order on
        /// scale up and strictly decreasing order on scale down, progressing only when
        /// the previous pod is ready or terminated. At most one pod will be changed
        /// at any time.
        OrderedReady = "OrderedReady",

        /// ParallelPodManagement will create and delete pods as soon as the stateful set
        /// replica count is changed, and will not wait for pods to be ready or complete
        /// termination.
        Parallel = "Parallel",
    }
}
