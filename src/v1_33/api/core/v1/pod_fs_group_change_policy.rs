// Generated from Go type k8s.io/api/core/v1.PodFSGroupChangePolicy

crate::string_enum! {
    /// PodFSGroupChangePolicy holds policies that will be used for applying fsGroup to a volume
    /// when volume is mounted.
    ///
    /// Used by:
    ///
    /// - [`PodSecurityContext::fs_group_change_policy`](crate::api::core::v1::PodSecurityContext::fs_group_change_policy)
    pub enum PodFSGroupChangePolicy {
        /// FSGroupChangeOnRootMismatch indicates that volume's ownership and permissions will be changed
        /// only when permission and ownership of root directory does not match with expected
        /// permissions on the volume. This can help shorten the time it takes to change
        /// ownership and permissions of a volume.
        OnRootMismatch = "OnRootMismatch",

        /// FSGroupChangeAlways indicates that volume's ownership and permissions
        /// should always be changed whenever volume is mounted inside a Pod. This the default
        /// behavior.
        Always = "Always",
    }
}
