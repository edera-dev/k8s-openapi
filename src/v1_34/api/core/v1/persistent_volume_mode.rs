// Generated from Go type k8s.io/api/core/v1.PersistentVolumeMode

crate::string_enum! {
    /// PersistentVolumeMode describes how a volume is intended to be consumed, either Block or Filesystem.
    ///
    /// Used by:
    ///
    /// - [`PersistentVolumeSpec::volume_mode`](crate::api::core::v1::PersistentVolumeSpec::volume_mode)
    /// - [`PersistentVolumeClaimSpec::volume_mode`](crate::api::core::v1::PersistentVolumeClaimSpec::volume_mode)
    pub enum PersistentVolumeMode {
        /// PersistentVolumeBlock means the volume will not be formatted with a filesystem and will remain a raw block device.
        Block = "Block",

        /// PersistentVolumeFilesystem means the volume will be or is formatted with a filesystem.
        Filesystem = "Filesystem",
    }
}
