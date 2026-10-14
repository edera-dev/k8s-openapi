// Generated from Go type k8s.io/api/core/v1.AzureDataDiskKind

crate::string_enum! {
    /// Used by:
    ///
    /// - [`AzureDiskVolumeSource::kind`](crate::api::core::v1::AzureDiskVolumeSource::kind)
    pub enum AzureDataDiskKind {
        Shared = "Shared",

        Dedicated = "Dedicated",

        Managed = "Managed",
    }
}
