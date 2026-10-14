// Generated from Go type k8s.io/api/core/v1.ScopeSelectorOperator

crate::string_enum! {
    /// A scope selector operator is the set of operators that can be used in
    /// a scope selector requirement.
    ///
    /// Used by:
    ///
    /// - [`ScopedResourceSelectorRequirement::operator`](crate::api::core::v1::ScopedResourceSelectorRequirement::operator)
    pub enum ScopeSelectorOperator {
        In = "In",

        NotIn = "NotIn",

        Exists = "Exists",

        DoesNotExist = "DoesNotExist",
    }
}
