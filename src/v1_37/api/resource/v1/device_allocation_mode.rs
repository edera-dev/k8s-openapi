// Generated from Go type k8s.io/api/resource/v1.DeviceAllocationMode

crate::string_enum! {
    /// Used by:
    ///
    /// - [`ExactDeviceRequest::allocation_mode`](crate::api::resource::v1::ExactDeviceRequest::allocation_mode)
    /// - [`DeviceSubRequest::allocation_mode`](crate::api::resource::v1::DeviceSubRequest::allocation_mode)
    pub enum DeviceAllocationMode {
        ExactCount = "ExactCount",

        All = "All",
    }
}
