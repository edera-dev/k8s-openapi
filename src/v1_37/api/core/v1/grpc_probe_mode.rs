// Generated from Go type k8s.io/api/core/v1.GRPCProbeMode

crate::string_enum! {
    /// GRPCProbeMode describes the connection mode for a gRPC probe.
    ///
    /// Used by:
    ///
    /// - [`GRPCAction::mode`](crate::api::core::v1::GRPCAction::mode)
    pub enum GRPCProbeMode {
        /// GRPCProbeModePlaintext indicates that the probe should use a plaintext
        /// (insecure) gRPC connection.
        Plaintext = "Plaintext",

        /// GRPCProbeModeTLS indicates that the probe should connect using TLS
        /// without certificate verification.
        TLS = "TLS",
    }
}
