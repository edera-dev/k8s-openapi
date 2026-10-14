// Generated from Go type k8s.io/api/core/v1.Protocol

crate::string_enum! {
    /// Protocol defines network protocols supported for things like container ports.
    ///
    /// Used by:
    ///
    /// - [`ContainerPort::protocol`](crate::api::core::v1::ContainerPort::protocol)
    /// - [`ServicePort::protocol`](crate::api::core::v1::ServicePort::protocol)
    /// - [`EndpointPort::protocol`](crate::api::core::v1::EndpointPort::protocol)
    /// - [`PortStatus::protocol`](crate::api::core::v1::PortStatus::protocol)
    /// - [`EndpointPort::protocol`](crate::api::discovery::v1::EndpointPort::protocol)
    /// - [`NetworkPolicyPort::protocol`](crate::api::networking::v1::NetworkPolicyPort::protocol)
    /// - [`IngressPortStatus::protocol`](crate::api::networking::v1::IngressPortStatus::protocol)
    pub enum Protocol {
        /// ProtocolTCP is the TCP protocol.
        TCP = "TCP",

        /// ProtocolUDP is the UDP protocol.
        UDP = "UDP",

        /// ProtocolSCTP is the SCTP protocol.
        SCTP = "SCTP",
    }
}
