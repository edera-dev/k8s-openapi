// Generated from Go type k8s.io/api/resource/v1.AllocationConfigSource

crate::string_enum! {
    /// Used by:
    ///
    /// - [`DeviceAllocationConfiguration::source`](crate::api::resource::v1::DeviceAllocationConfiguration::source)
    pub enum AllocationConfigSource {
        FromClass = "FromClass",

        FromClaim = "FromClaim",
    }
}
