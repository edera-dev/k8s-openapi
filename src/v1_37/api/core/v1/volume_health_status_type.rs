// Generated from Go type k8s.io/api/core/v1.VolumeHealthStatusType

crate::string_enum! {
    /// VolumeHealthStatusType describes the health status category of a volume.
    ///
    /// Used by:
    ///
    /// - [`VolumeHealthCondition::status`](crate::api::core::v1::VolumeHealthCondition::status)
    pub enum VolumeHealthStatusType {
        /// VolumeHealthInaccessible indicates the volume cannot be accessed.
        Inaccessible = "Inaccessible",

        /// VolumeHealthDataLoss indicates data loss has been detected on the volume.
        DataLoss = "DataLoss",

        /// VolumeHealthDegraded indicates the volume is functioning but with reduced capability.
        Degraded = "Degraded",
    }
}
