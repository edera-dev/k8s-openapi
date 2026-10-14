// Generated from Go type k8s.io/api/apps/v1.StatefulSetUpdateStrategyType

crate::string_enum! {
    /// StatefulSetUpdateStrategyType is a string enumeration type that enumerates
    /// all possible update strategies for the StatefulSet controller.
    ///
    /// Used by:
    ///
    /// - [`StatefulSetUpdateStrategy::type_`](crate::api::apps::v1::StatefulSetUpdateStrategy::type_)
    pub enum StatefulSetUpdateStrategyType {
        /// RollingUpdateStatefulSetStrategyType indicates that update will be
        /// applied to all Pods in the StatefulSet with respect to the StatefulSet
        /// ordering constraints. When a scale operation is performed with this
        /// strategy, new Pods will be created from the specification version indicated
        /// by the StatefulSet's updateRevision.
        RollingUpdate = "RollingUpdate",

        /// OnDeleteStatefulSetStrategyType triggers the legacy behavior. Version
        /// tracking and ordered rolling restarts are disabled. Pods are recreated
        /// from the StatefulSetSpec when they are manually deleted. When a scale
        /// operation is performed with this strategy,specification version indicated
        /// by the StatefulSet's currentRevision.
        OnDelete = "OnDelete",
    }
}
