// Generated from Go type k8s.io/api/networking/v1.PolicyType

crate::string_enum! {
    /// PolicyType string describes the NetworkPolicy type
    /// This type is beta-level in 1.8
    ///
    /// Used by:
    ///
    /// - [`NetworkPolicySpec::policy_types`](crate::api::networking::v1::NetworkPolicySpec::policy_types)
    pub enum PolicyType {
        /// PolicyTypeIngress is a NetworkPolicy that affects ingress traffic on selected pods
        Ingress = "Ingress",

        /// PolicyTypeEgress is a NetworkPolicy that affects egress traffic on selected pods
        Egress = "Egress",
    }
}
