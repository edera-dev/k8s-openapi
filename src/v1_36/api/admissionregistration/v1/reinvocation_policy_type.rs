// Generated from Go type k8s.io/api/admissionregistration/v1.ReinvocationPolicyType

crate::string_enum! {
    /// ReinvocationPolicyType specifies what type of policy is used when other admission plugins also perform
    /// modifications.
    ///
    /// Used by:
    ///
    /// - [`MutatingWebhook::reinvocation_policy`](crate::api::admissionregistration::v1::MutatingWebhook::reinvocation_policy)
    /// - [`MutatingAdmissionPolicySpec::reinvocation_policy`](crate::api::admissionregistration::v1::MutatingAdmissionPolicySpec::reinvocation_policy)
    /// - [`MutatingAdmissionPolicySpec::reinvocation_policy`](crate::api::admissionregistration::v1alpha1::MutatingAdmissionPolicySpec::reinvocation_policy)
    /// - [`MutatingAdmissionPolicySpec::reinvocation_policy`](crate::api::admissionregistration::v1beta1::MutatingAdmissionPolicySpec::reinvocation_policy)
    pub enum ReinvocationPolicyType {
        /// NeverReinvocationPolicy indicates that the mutation must not be called more than once in a
        /// single admission evaluation.
        Never = "Never",

        /// IfNeededReinvocationPolicy indicates that the mutation may be called at least one
        /// additional time as part of the admission evaluation if the object being admitted is
        /// modified by other admission plugins after the initial mutation call.
        IfNeeded = "IfNeeded",
    }
}
