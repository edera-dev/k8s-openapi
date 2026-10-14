// Generated from Go type k8s.io/api/core/v1.AppArmorProfileType

crate::string_enum! {
    /// Used by:
    ///
    /// - [`AppArmorProfile::type_`](crate::api::core::v1::AppArmorProfile::type_)
    pub enum AppArmorProfileType {
        /// AppArmorProfileTypeUnconfined indicates that no AppArmor profile should be enforced.
        Unconfined = "Unconfined",

        /// AppArmorProfileTypeRuntimeDefault indicates that the container runtime's default AppArmor
        /// profile should be used.
        RuntimeDefault = "RuntimeDefault",

        /// AppArmorProfileTypeLocalhost indicates that a profile pre-loaded on the node should be used.
        Localhost = "Localhost",
    }
}
