// Generated from Go type k8s.io/api/apps/v1.DaemonSetUpdateStrategyType

crate::string_enum! {
    /// Used by:
    ///
    /// - [`DaemonSetUpdateStrategy::type_`](crate::api::apps::v1::DaemonSetUpdateStrategy::type_)
    pub enum DaemonSetUpdateStrategyType {
        /// Replace the old daemons by new ones using rolling update i.e replace them on each node one after the other.
        RollingUpdate = "RollingUpdate",

        /// Replace the old daemons only when it's killed
        OnDelete = "OnDelete",
    }
}
