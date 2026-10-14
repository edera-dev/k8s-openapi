// Generated from Go type k8s.io/api/core/v1.PersistentVolumeClaimModifyVolumeStatus

crate::string_enum! {
    /// New statuses can be added in the future. Consumers should check for unknown statuses and fail appropriately
    ///
    /// Used by:
    ///
    /// - [`ModifyVolumeStatus::status`](crate::api::core::v1::ModifyVolumeStatus::status)
    pub enum PersistentVolumeClaimModifyVolumeStatus {
        /// Pending indicates that the PersistentVolumeClaim cannot be modified due to unmet requirements, such as
        /// the specified VolumeAttributesClass not existing
        Pending = "Pending",

        /// InProgress indicates that the volume is being modified
        InProgress = "InProgress",

        /// Infeasible indicates that the request has been rejected as invalid by the CSI driver. To
        /// resolve the error, a valid VolumeAttributesClass needs to be specified
        Infeasible = "Infeasible",
    }
}
