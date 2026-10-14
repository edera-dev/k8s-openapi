// Generated from Go type k8s.io/api/core/v1.URIScheme

crate::string_enum! {
    /// URIScheme identifies the scheme used for connection to a host for Get actions
    ///
    /// Used by:
    ///
    /// - [`HTTPGetAction::scheme`](crate::api::core::v1::HTTPGetAction::scheme)
    pub enum URIScheme {
        /// URISchemeHTTP means that the scheme used will be http://
        HTTP = "HTTP",

        /// URISchemeHTTPS means that the scheme used will be https://
        HTTPS = "HTTPS",
    }
}
