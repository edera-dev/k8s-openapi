// Generated from Go type k8s.io/api/core/v1.ServiceAffinity

crate::string_enum! {
    /// Session Affinity Type string
    ///
    /// Used by:
    ///
    /// - [`ServiceSpec::session_affinity`](crate::api::core::v1::ServiceSpec::session_affinity)
    pub enum ServiceAffinity {
        /// ServiceAffinityClientIP is the Client IP based.
        ClientIP = "ClientIP",

        /// ServiceAffinityNone - no session affinity.
        None = "None",
    }
}
