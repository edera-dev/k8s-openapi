// Generated from Go type k8s.io/api/core/v1.NamespacePhase

crate::string_enum! {
    /// Used by:
    ///
    /// - [`NamespaceStatus::phase`](crate::api::core::v1::NamespaceStatus::phase)
    pub enum NamespacePhase {
        /// NamespaceActive means the namespace is available for use in the system
        Active = "Active",

        /// NamespaceTerminating means the namespace is undergoing graceful termination
        Terminating = "Terminating",
    }
}
