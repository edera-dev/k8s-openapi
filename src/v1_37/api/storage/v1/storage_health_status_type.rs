// Generated from Go type k8s.io/api/storage/v1.StorageHealthStatusType

crate::string_enum! {
    /// StorageHealthStatusType describes the health status category of a storage backend.
    ///
    /// Used by:
    ///
    /// - [`StorageHealthCondition::status`](crate::api::storage::v1::StorageHealthCondition::status)
    pub enum StorageHealthStatusType {
        /// StorageUnreachable indicates the storage backend is unreachable.
        StorageUnreachable = "StorageUnreachable",

        /// StorageDegraded indicates the storage backend is functioning with reduced capability.
        StorageDegraded = "StorageDegraded",
    }
}
