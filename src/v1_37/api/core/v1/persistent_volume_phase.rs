// Generated from Go type k8s.io/api/core/v1.PersistentVolumePhase

crate::string_enum! {
    /// Used by:
    ///
    /// - [`PersistentVolumeStatus::phase`](crate::api::core::v1::PersistentVolumeStatus::phase)
    pub enum PersistentVolumePhase {
        /// used for PersistentVolumes that are not available
        Pending = "Pending",

        /// used for PersistentVolumes that are not yet bound
        /// Available volumes are held by the binder and matched to PersistentVolumeClaims
        Available = "Available",

        /// used for PersistentVolumes that are bound
        Bound = "Bound",

        /// used for PersistentVolumes where the bound PersistentVolumeClaim was deleted
        /// released volumes must be recycled before becoming available again
        /// this phase is used by the persistent volume claim binder to signal to another process to reclaim the resource
        Released = "Released",

        /// used for PersistentVolumes that failed to be correctly recycled or deleted after being released from a claim
        Failed = "Failed",
    }
}
