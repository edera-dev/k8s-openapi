// Generated from Go type k8s.io/api/core/v1.PodQOSClass

crate::string_enum! {
    /// PodQOSClass defines the supported qos classes of Pods.
    ///
    /// Used by:
    ///
    /// - [`PodStatus::qos_class`](crate::api::core::v1::PodStatus::qos_class)
    pub enum PodQOSClass {
        /// PodQOSGuaranteed is the Guaranteed qos class.
        Guaranteed = "Guaranteed",

        /// PodQOSBurstable is the Burstable qos class.
        Burstable = "Burstable",

        /// PodQOSBestEffort is the BestEffort qos class.
        BestEffort = "BestEffort",
    }
}
