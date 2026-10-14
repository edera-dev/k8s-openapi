// Generated from Go type k8s.io/api/core/v1.IPFamily

crate::string_enum! {
    /// IPFamily represents the IP Family (IPv4 or IPv6). This type is used
    /// to express the family of an IP expressed by a type (e.g. service.spec.ipFamilies).
    ///
    /// Used by:
    ///
    /// - [`ServiceSpec::ip_families`](crate::api::core::v1::ServiceSpec::ip_families)
    pub enum IPFamily {
        /// IPv4Protocol indicates that this IP is IPv4 protocol
        IPv4 = "IPv4",

        /// IPv6Protocol indicates that this IP is IPv6 protocol
        IPv6 = "IPv6",

        /// IPFamilyUnknown indicates that this IP is unknown protocol
        IPFamilyUnknown = "",
    }
}
