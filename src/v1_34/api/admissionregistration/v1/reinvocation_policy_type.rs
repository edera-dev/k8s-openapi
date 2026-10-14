// Generated from Go type k8s.io/api/admissionregistration/v1.ReinvocationPolicyType

crate::string_enum! {
    /// ReinvocationPolicyType specifies what type of policy the admission hook uses.
    ///
    /// Used by:
    ///
    /// - [`MutatingWebhook::reinvocation_policy`](crate::api::admissionregistration::v1::MutatingWebhook::reinvocation_policy)
    /// - [`MutatingAdmissionPolicySpec::reinvocation_policy`](crate::api::admissionregistration::v1alpha1::MutatingAdmissionPolicySpec::reinvocation_policy)
    /// - [`MutatingAdmissionPolicySpec::reinvocation_policy`](crate::api::admissionregistration::v1beta1::MutatingAdmissionPolicySpec::reinvocation_policy)
    pub enum ReinvocationPolicyType {
        /// NeverReinvocationPolicy indicates that the webhook must not be called more than once in a
        /// single admission evaluation.
        Never = "Never",

        /// IfNeededReinvocationPolicy indicates that the webhook may be called at least one
        /// additional time as part of the admission evaluation if the object being admitted is
        /// modified by other admission plugins after the initial webhook call.
        IfNeeded = "IfNeeded",
    }
}
