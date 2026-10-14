// Generated from Go type k8s.io/api/core/v1.PersistentVolumeClaimPhase

crate::string_enum! {
    /// Used by:
    ///
    /// - [`PersistentVolumeClaimStatus::phase`](crate::api::core::v1::PersistentVolumeClaimStatus::phase)
    pub enum PersistentVolumeClaimPhase {
        /// used for PersistentVolumeClaims that are not yet bound
        Pending = "Pending",

        /// used for PersistentVolumeClaims that are bound
        Bound = "Bound",

        /// used for PersistentVolumeClaims that lost their underlying
        /// PersistentVolume. The claim was bound to a PersistentVolume and this
        /// volume does not exist any longer and all data on it was lost.
        Lost = "Lost",
    }
}
