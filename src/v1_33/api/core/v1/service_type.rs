// Generated from Go type k8s.io/api/core/v1.ServiceType

crate::string_enum! {
    /// Service Type string describes ingress methods for a service
    ///
    /// Used by:
    ///
    /// - [`ServiceSpec::type_`](crate::api::core::v1::ServiceSpec::type_)
    pub enum ServiceType {
        /// ServiceTypeClusterIP means a service will only be accessible inside the
        /// cluster, via the cluster IP.
        ClusterIP = "ClusterIP",

        /// ServiceTypeNodePort means a service will be exposed on one port of
        /// every node, in addition to 'ClusterIP' type.
        NodePort = "NodePort",

        /// ServiceTypeLoadBalancer means a service will be exposed via an
        /// external load balancer (if the cloud provider supports it), in addition
        /// to 'NodePort' type.
        LoadBalancer = "LoadBalancer",

        /// ServiceTypeExternalName means a service consists of only a reference to
        /// an external name that kubedns or equivalent will return as a CNAME
        /// record, with no exposing or proxying of any pods involved.
        ExternalName = "ExternalName",
    }
}
