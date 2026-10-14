// Generated from Go type k8s.io/api/core/v1.PersistentVolumeAccessMode

crate::string_enum! {
    /// Used by:
    ///
    /// - [`PersistentVolumeSpec::access_modes`](crate::api::core::v1::PersistentVolumeSpec::access_modes)
    /// - [`PersistentVolumeClaimSpec::access_modes`](crate::api::core::v1::PersistentVolumeClaimSpec::access_modes)
    /// - [`PersistentVolumeClaimStatus::access_modes`](crate::api::core::v1::PersistentVolumeClaimStatus::access_modes)
    /// - [`StorageHealthCondition::access_mode`](crate::api::storage::v1::StorageHealthCondition::access_mode)
    pub enum PersistentVolumeAccessMode {
        /// can be mounted in read/write mode to exactly 1 host
        ReadWriteOnce = "ReadWriteOnce",

        /// can be mounted in read-only mode to many hosts
        ReadOnlyMany = "ReadOnlyMany",

        /// can be mounted in read/write mode to many hosts
        ReadWriteMany = "ReadWriteMany",

        /// can be mounted in read/write mode to exactly 1 pod
        /// cannot be used in combination with other access modes
        ReadWriteOncePod = "ReadWriteOncePod",
    }
}
