// Generated from Go type k8s.io/api/core/v1.HTTPProtocol

crate::string_enum! {
    /// HTTPProtocol selects the wire protocol for the HTTP probe,
    /// independently of the URI scheme.
    ///
    /// Used by:
    ///
    /// - [`HTTPGetAction::protocol`](crate::api::core::v1::HTTPGetAction::protocol)
    pub enum HTTPProtocol {
        /// HTTPProtocolHTTP1 uses HTTP/1.1 (the existing default).
        HTTP1 = "HTTP1",

        /// HTTPProtocolHTTP2 uses HTTP/2.
        /// Currently, only cleartext with prior knowledge (h2c) is supported, and must be used with scheme HTTP.
        HTTP2 = "HTTP2",
    }
}
