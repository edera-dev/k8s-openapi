// Generated from Go type k8s.io/api/resource/v1beta1.DeviceTolerationOperator

crate::string_enum! {
    /// A toleration operator is the set of operators that can be used in a toleration.
    ///
    /// Used by:
    ///
    /// - [`DeviceToleration::operator`](crate::api::resource::v1beta1::DeviceToleration::operator)
    pub enum DeviceTolerationOperator {
        Exists = "Exists",

        Equal = "Equal",
    }
}
