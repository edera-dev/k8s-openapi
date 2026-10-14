// Generated from Go type k8s.io/api/resource/v1beta1.DeviceAllocationMode

crate::string_enum! {
    /// Used by:
    ///
    /// - [`DeviceRequest::allocation_mode`](crate::api::resource::v1beta1::DeviceRequest::allocation_mode)
    /// - [`DeviceSubRequest::allocation_mode`](crate::api::resource::v1beta1::DeviceSubRequest::allocation_mode)
    pub enum DeviceAllocationMode {
        ExactCount = "ExactCount",

        All = "All",
    }
}
