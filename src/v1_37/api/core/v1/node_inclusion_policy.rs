// Generated from Go type k8s.io/api/core/v1.NodeInclusionPolicy

crate::string_enum! {
    /// NodeInclusionPolicy defines the type of node inclusion policy
    ///
    /// Used by:
    ///
    /// - [`TopologySpreadConstraint::node_affinity_policy`](crate::api::core::v1::TopologySpreadConstraint::node_affinity_policy)
    /// - [`TopologySpreadConstraint::node_taints_policy`](crate::api::core::v1::TopologySpreadConstraint::node_taints_policy)
    pub enum NodeInclusionPolicy {
        /// NodeInclusionPolicyIgnore means ignore this scheduling directive when calculating pod topology spread skew.
        Ignore = "Ignore",

        /// NodeInclusionPolicyHonor means use this scheduling directive when calculating pod topology spread skew.
        Honor = "Honor",
    }
}
