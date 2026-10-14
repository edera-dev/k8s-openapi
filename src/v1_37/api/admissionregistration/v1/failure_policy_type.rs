// Generated from Go type k8s.io/api/admissionregistration/v1.FailurePolicyType

crate::string_enum! {
    /// FailurePolicyType specifies a failure policy that defines how unrecognized errors from the admission endpoint are handled.
    ///
    /// Used by:
    ///
    /// - [`ValidatingAdmissionPolicySpec::failure_policy`](crate::api::admissionregistration::v1::ValidatingAdmissionPolicySpec::failure_policy)
    /// - [`ValidatingWebhook::failure_policy`](crate::api::admissionregistration::v1::ValidatingWebhook::failure_policy)
    /// - [`MutatingWebhook::failure_policy`](crate::api::admissionregistration::v1::MutatingWebhook::failure_policy)
    /// - [`MutatingAdmissionPolicySpec::failure_policy`](crate::api::admissionregistration::v1::MutatingAdmissionPolicySpec::failure_policy)
    pub enum FailurePolicyType {
        /// Ignore means that an error calling the webhook is ignored.
        Ignore = "Ignore",

        /// Fail means that an error calling the webhook causes the admission to fail.
        Fail = "Fail",
    }
}
