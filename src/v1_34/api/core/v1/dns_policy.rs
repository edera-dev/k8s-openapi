// Generated from Go type k8s.io/api/core/v1.DNSPolicy

crate::string_enum! {
    /// DNSPolicy defines how a pod's DNS will be configured.
    ///
    /// Used by:
    ///
    /// - [`PodSpec::dns_policy`](crate::api::core::v1::PodSpec::dns_policy)
    pub enum DNSPolicy {
        /// DNSClusterFirstWithHostNet indicates that the pod should use cluster DNS
        /// first, if it is available, then fall back on the default
        /// (as determined by kubelet) DNS settings.
        ClusterFirstWithHostNet = "ClusterFirstWithHostNet",

        /// DNSClusterFirst indicates that the pod should use cluster DNS
        /// first unless hostNetwork is true, if it is available, then
        /// fall back on the default (as determined by kubelet) DNS settings.
        ClusterFirst = "ClusterFirst",

        /// DNSDefault indicates that the pod should use the default (as
        /// determined by kubelet) DNS settings.
        Default = "Default",

        /// DNSNone indicates that the pod should use empty DNS settings. DNS
        /// parameters such as nameservers and search paths should be defined via
        /// DNSConfig.
        None = "None",
    }
}
