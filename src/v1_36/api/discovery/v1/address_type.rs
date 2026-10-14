// Generated from Go type k8s.io/api/discovery/v1.AddressType

crate::string_enum! {
    /// AddressType represents the type of address referred to by an endpoint.
    ///
    /// Used by:
    ///
    /// - [`EndpointSlice::address_type`](crate::api::discovery::v1::EndpointSlice::address_type)
    pub enum AddressType {
        /// AddressTypeIPv4 represents an IPv4 Address.
        IPv4 = "IPv4",

        /// AddressTypeIPv6 represents an IPv6 Address.
        IPv6 = "IPv6",

        /// AddressTypeFQDN represents a FQDN.
        FQDN = "FQDN",
    }
}
