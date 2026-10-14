// Generated from Go type k8s.io/api/admissionregistration/v1.MatchPolicyType

crate::string_enum! {
    /// MatchPolicyType specifies the type of match policy.
    ///
    /// Used by:
    ///
    /// - [`MatchResources::match_policy`](crate::api::admissionregistration::v1::MatchResources::match_policy)
    /// - [`ValidatingWebhook::match_policy`](crate::api::admissionregistration::v1::ValidatingWebhook::match_policy)
    /// - [`MutatingWebhook::match_policy`](crate::api::admissionregistration::v1::MutatingWebhook::match_policy)
    pub enum MatchPolicyType {
        /// Exact means requests should only be sent to the webhook if they exactly match a given rule.
        Exact = "Exact",

        /// Equivalent means requests should be sent to the webhook if they modify a resource listed in rules via another API group or version.
        Equivalent = "Equivalent",
    }
}
