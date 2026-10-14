// Generated from Go type k8s.io/api/admissionregistration/v1alpha1.ParameterNotFoundActionType

crate::string_enum! {
    /// ParameterNotFoundActionType specifies a failure policy that defines how a binding
    /// is evaluated when the param referred by its perNamespaceParamRef is not found.
    ///
    /// Used by:
    ///
    /// - [`ParamRef::parameter_not_found_action`](crate::api::admissionregistration::v1alpha1::ParamRef::parameter_not_found_action)
    pub enum ParameterNotFoundActionType {
        /// Ignore means that an error finding params for a binding is ignored
        Allow = "Allow",

        /// Fail means that an error finding params for a binding is ignored
        Deny = "Deny",
    }
}
