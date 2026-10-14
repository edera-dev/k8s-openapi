// Generated from Go type k8s.io/api/batch/v1.PodFailurePolicyOnExitCodesOperator

crate::string_enum! {
    /// Used by:
    ///
    /// - [`PodFailurePolicyOnExitCodesRequirement::operator`](crate::api::batch::v1::PodFailurePolicyOnExitCodesRequirement::operator)
    pub enum PodFailurePolicyOnExitCodesOperator {
        In = "In",

        NotIn = "NotIn",
    }
}
