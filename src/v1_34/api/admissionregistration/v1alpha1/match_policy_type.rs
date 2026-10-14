// Generated from Go type k8s.io/api/admissionregistration/v1alpha1.MatchPolicyType

crate::string_enum! {
    /// MatchPolicyType specifies the type of match policy.
    ///
    /// Used by:
    ///
    /// - [`MatchResources::match_policy`](crate::api::admissionregistration::v1alpha1::MatchResources::match_policy)
    pub enum MatchPolicyType {
        /// Exact means requests should only be sent to the admission webhook or admission policy if they exactly match a given rule.
        Exact = "Exact",

        /// Equivalent means requests should be sent to the admission webhook or admission policy if they modify a resource listed
        /// in rules via an equivalent API group or version. For example, `autoscaling/v1` and `autoscaling/v2`
        /// HorizontalPodAutoscalers are equivalent: the same set of resources appear via both APIs.
        Equivalent = "Equivalent",
    }
}
