// Generated from Go type k8s.io/api/core/v1.PersistentVolumeReclaimPolicy

crate::string_enum! {
    /// PersistentVolumeReclaimPolicy describes a policy for end-of-life maintenance of persistent volumes.
    ///
    /// Used by:
    ///
    /// - [`PersistentVolumeSpec::persistent_volume_reclaim_policy`](crate::api::core::v1::PersistentVolumeSpec::persistent_volume_reclaim_policy)
    /// - [`StorageClass::reclaim_policy`](crate::api::storage::v1::StorageClass::reclaim_policy)
    pub enum PersistentVolumeReclaimPolicy {
        /// PersistentVolumeReclaimRecycle means the volume will be recycled back into the pool of unbound persistent volumes on release from its claim.
        /// The volume plugin must support Recycling.
        Recycle = "Recycle",

        /// PersistentVolumeReclaimDelete means the volume will be deleted from Kubernetes on release from its claim.
        /// The volume plugin must support Deletion.
        Delete = "Delete",

        /// PersistentVolumeReclaimRetain means the volume will be left in its current phase (Released) for manual reclamation by the administrator.
        /// The default policy is Retain.
        Retain = "Retain",
    }
}
