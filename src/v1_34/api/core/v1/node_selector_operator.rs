// Generated from Go type k8s.io/api/core/v1.NodeSelectorOperator

crate::string_enum! {
    /// A node selector operator is the set of operators that can be used in
    /// a node selector requirement.
    ///
    /// Used by:
    ///
    /// - [`NodeSelectorRequirement::operator`](crate::api::core::v1::NodeSelectorRequirement::operator)
    pub enum NodeSelectorOperator {
        In = "In",

        NotIn = "NotIn",

        Exists = "Exists",

        DoesNotExist = "DoesNotExist",

        Gt = "Gt",

        Lt = "Lt",
    }
}
