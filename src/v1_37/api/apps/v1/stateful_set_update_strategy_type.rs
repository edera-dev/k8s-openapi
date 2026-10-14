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

        /// OnDeleteStatefulSetStrategyType disables ordered rolling restarts. Version
        /// tracking is done on a best-effort basis - the controller will try to
        /// eventually converge StatefulSet's currentRevision with updateRevision.
        /// Pods are recreated from the StatefulSetSpec when they are manually deleted.
        /// When a scale operation is performed with this strategy, new Pods will be
        /// created from the specification version indicated by the StatefulSet's updateRevision.
        OnDelete = "OnDelete",

        /// RecreateStatefulSetStrategyType indicates that all existing pods will be
        /// deleted and fully terminated before any new-revision pods are created.
        /// This ensures that old and new revision Pods never run at the same time.
        /// This is an alpha type and requires enabling StatefulSetRecreateStrategy feature gate.
        /// Switching to the Recreate strategy is allowed when the feature gate is enabled,
        /// and switching away from it to a different update strategy is also allowed.
        Recreate = "Recreate",
    }
}
