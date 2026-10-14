// Generated from Go type k8s.io/api/admissionregistration/v1alpha1.FailurePolicyType

crate::string_enum! {
    /// FailurePolicyType specifies a failure policy that defines how unrecognized errors from the admission endpoint are handled.
    ///
    /// Used by:
    ///
    /// - [`MutatingAdmissionPolicySpec::failure_policy`](crate::api::admissionregistration::v1alpha1::MutatingAdmissionPolicySpec::failure_policy)
    pub enum FailurePolicyType {
        /// Ignore means that an error calling the admission webhook or admission policy is ignored.
        Ignore = "Ignore",

        /// Fail means that an error calling the admission webhook or admission policy causes resource admission to fail.
        Fail = "Fail",
    }
}
