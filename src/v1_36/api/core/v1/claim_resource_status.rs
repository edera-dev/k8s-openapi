// Generated from Go type k8s.io/api/core/v1.ClaimResourceStatus

crate::string_enum! {
    /// When a controller receives persistentvolume claim update with ClaimResourceStatus for a resource
    /// that it does not recognizes, then it should ignore that update and let other controllers
    /// handle it.
    ///
    /// Used by:
    ///
    /// - [`PersistentVolumeClaimStatus::allocated_resource_statuses`](crate::api::core::v1::PersistentVolumeClaimStatus::allocated_resource_statuses)
    pub enum ClaimResourceStatus {
        /// State set when resize controller starts resizing the volume in control-plane.
        ControllerResizeInProgress = "ControllerResizeInProgress",

        /// State set when resize has failed in resize controller with a terminal unrecoverable error.
        /// Transient errors such as timeout should not set this status and should leave allocatedResourceStatus
        /// unmodified, so as resize controller can resume the volume expansion.
        ControllerResizeInfeasible = "ControllerResizeInfeasible",

        /// State set when resize controller has finished resizing the volume but further resizing of volume
        /// is needed on the node.
        NodeResizePending = "NodeResizePending",

        /// State set when kubelet starts resizing the volume.
        NodeResizeInProgress = "NodeResizeInProgress",

        /// State set when resizing has failed in kubelet with a terminal unrecoverable error. Transient errors
        /// shouldn't set this status
        NodeResizeInfeasible = "NodeResizeInfeasible",
    }
}
