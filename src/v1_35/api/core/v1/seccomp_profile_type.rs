// Generated from Go type k8s.io/api/core/v1.SeccompProfileType

crate::string_enum! {
    /// SeccompProfileType defines the supported seccomp profile types.
    ///
    /// Used by:
    ///
    /// - [`SeccompProfile::type_`](crate::api::core::v1::SeccompProfile::type_)
    pub enum SeccompProfileType {
        /// SeccompProfileTypeUnconfined indicates no seccomp profile is applied (A.K.A. unconfined).
        Unconfined = "Unconfined",

        /// SeccompProfileTypeRuntimeDefault represents the default container runtime seccomp profile.
        RuntimeDefault = "RuntimeDefault",

        /// SeccompProfileTypeLocalhost indicates a profile defined in a file on the node should be used.
        /// The file's location relative to \<kubelet-root-dir\>/seccomp.
        Localhost = "Localhost",
    }
}
