// Generated from Go type k8s.io/api/core/v1.SupplementalGroupsPolicy

crate::string_enum! {
    /// SupplementalGroupsPolicy defines how supplemental groups
    /// of the first container processes are calculated.
    ///
    /// Used by:
    ///
    /// - [`PodSecurityContext::supplemental_groups_policy`](crate::api::core::v1::PodSecurityContext::supplemental_groups_policy)
    pub enum SupplementalGroupsPolicy {
        /// SupplementalGroupsPolicyMerge means that the container's provided
        /// SupplementalGroups and FsGroup (specified in SecurityContext) will be
        /// merged with the primary user's groups as defined in the container image
        /// (in /etc/group).
        Merge = "Merge",

        /// SupplementalGroupsPolicyStrict means that the container's provided
        /// SupplementalGroups and FsGroup (specified in SecurityContext) will be
        /// used instead of any groups defined in the container image.
        Strict = "Strict",
    }
}
