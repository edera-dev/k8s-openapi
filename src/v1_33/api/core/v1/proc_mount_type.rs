// Generated from Go type k8s.io/api/core/v1.ProcMountType

crate::string_enum! {
    /// Used by:
    ///
    /// - [`SecurityContext::proc_mount`](crate::api::core::v1::SecurityContext::proc_mount)
    pub enum ProcMountType {
        /// DefaultProcMount uses the container runtime defaults for readonly and masked
        /// paths for /proc.  Most container runtimes mask certain paths in /proc to avoid
        /// accidental security exposure of special devices or information.
        Default = "Default",

        /// UnmaskedProcMount bypasses the default masking behavior of the container
        /// runtime and ensures the newly created /proc the container stays in tact with
        /// no modifications.
        Unmasked = "Unmasked",
    }
}
