// Generated from Go type k8s.io/api/core/v1.ServiceExternalTrafficPolicy

crate::string_enum! {
    /// ServiceExternalTrafficPolicy describes how nodes distribute service traffic they
    /// receive on one of the Service's "externally-facing" addresses (NodePorts, ExternalIPs,
    /// and LoadBalancer IPs.
    ///
    /// Used by:
    ///
    /// - [`ServiceSpec::external_traffic_policy`](crate::api::core::v1::ServiceSpec::external_traffic_policy)
    pub enum ServiceExternalTrafficPolicy {
        /// ServiceExternalTrafficPolicyCluster routes traffic to all endpoints.
        Cluster = "Cluster",

        /// ServiceExternalTrafficPolicyLocal preserves the source IP of the traffic by
        /// routing only to endpoints on the same node as the traffic was received on
        /// (dropping the traffic if there are no local endpoints).
        Local = "Local",
    }
}
