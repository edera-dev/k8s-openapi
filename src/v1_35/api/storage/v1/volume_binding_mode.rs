// Generated from Go type k8s.io/api/storage/v1.VolumeBindingMode

crate::string_enum! {
    /// VolumeBindingMode indicates how PersistentVolumeClaims should be bound.
    ///
    /// Used by:
    ///
    /// - [`StorageClass::volume_binding_mode`](crate::api::storage::v1::StorageClass::volume_binding_mode)
    pub enum VolumeBindingMode {
        /// VolumeBindingImmediate indicates that PersistentVolumeClaims should be
        /// immediately provisioned and bound.  This is the default mode.
        Immediate = "Immediate",

        /// VolumeBindingWaitForFirstConsumer indicates that PersistentVolumeClaims
        /// should not be provisioned and bound until the first Pod is created that
        /// references the PeristentVolumeClaim.  The volume provisioning and
        /// binding will occur during Pod scheduing.
        WaitForFirstConsumer = "WaitForFirstConsumer",
    }
}
