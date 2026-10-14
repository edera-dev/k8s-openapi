// Generated from Go type k8s.io/api/core/v1.ServiceInternalTrafficPolicy

crate::string_enum! {
    /// ServiceInternalTrafficPolicy describes how nodes distribute service traffic they
    /// receive on the ClusterIP.
    ///
    /// Used by:
    ///
    /// - [`ServiceSpec::internal_traffic_policy`](crate::api::core::v1::ServiceSpec::internal_traffic_policy)
    pub enum ServiceInternalTrafficPolicy {
        /// ServiceInternalTrafficPolicyCluster routes traffic to all endpoints.
        Cluster = "Cluster",

        /// ServiceInternalTrafficPolicyLocal routes traffic only to endpoints on the same
        /// node as the client pod (dropping the traffic if there are no local endpoints).
        Local = "Local",
    }
}
