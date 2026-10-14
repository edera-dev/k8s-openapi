// Generated from Go type k8s.io/api/admissionregistration/v1.ValidationAction

crate::string_enum! {
    /// ValidationAction specifies a policy enforcement action.
    ///
    /// Used by:
    ///
    /// - [`ValidatingAdmissionPolicyBindingSpec::validation_actions`](crate::api::admissionregistration::v1::ValidatingAdmissionPolicyBindingSpec::validation_actions)
    pub enum ValidationAction {
        /// Deny specifies that a validation failure results in a denied request.
        Deny = "Deny",

        /// Warn specifies that a validation failure is reported to the request client
        /// in HTTP Warning headers, with a warning code of 299. Warnings can be sent
        /// both for allowed or denied admission responses.
        Warn = "Warn",

        /// Audit specifies that a validation failure is included in the published
        /// audit event for the request. The audit event will contain a
        /// `validation.policy.admission.k8s.io/validation_failure` audit annotation
        /// with a value containing the details of the validation failure.
        Audit = "Audit",
    }
}
