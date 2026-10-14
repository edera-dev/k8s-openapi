// Generated from Go type k8s.io/api/resource/v1alpha3.DeviceTaintEffect

crate::string_enum! {
    /// Used by:
    ///
    /// - [`DeviceTaint::effect`](crate::api::resource::v1alpha3::DeviceTaint::effect)
    /// - [`DeviceToleration::effect`](crate::api::resource::v1alpha3::DeviceToleration::effect)
    pub enum DeviceTaintEffect {
        /// Do not allow new pods to schedule which use a tainted device unless they tolerate the taint,
        /// but allow all pods submitted to Kubelet without going through the scheduler
        /// to start, and allow all already-running pods to continue running.
        NoSchedule = "NoSchedule",

        /// Evict any already-running pods that do not tolerate the device taint.
        NoExecute = "NoExecute",
    }
}
