// Generated from Go type k8s.io/api/core/v1.TolerationOperator

crate::string_enum! {
    /// A toleration operator is the set of operators that can be used in a toleration.
    ///
    /// Used by:
    ///
    /// - [`Toleration::operator`](crate::api::core::v1::Toleration::operator)
    pub enum TolerationOperator {
        Exists = "Exists",

        Equal = "Equal",

        Lt = "Lt",

        Gt = "Gt",
    }
}
