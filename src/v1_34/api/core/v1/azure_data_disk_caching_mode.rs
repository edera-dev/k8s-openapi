// Generated from Go type k8s.io/api/core/v1.AzureDataDiskCachingMode

crate::string_enum! {
    /// Used by:
    ///
    /// - [`AzureDiskVolumeSource::caching_mode`](crate::api::core::v1::AzureDiskVolumeSource::caching_mode)
    pub enum AzureDataDiskCachingMode {
        None = "None",

        ReadOnly = "ReadOnly",

        ReadWrite = "ReadWrite",
    }
}
