// Generated from Go type k8s.io/api/core/v1.ResourceQuotaScope

crate::string_enum! {
    /// A ResourceQuotaScope defines a filter that must match each object tracked by a quota
    ///
    /// Used by:
    ///
    /// - [`ResourceQuotaSpec::scopes`](crate::api::core::v1::ResourceQuotaSpec::scopes)
    /// - [`ScopedResourceSelectorRequirement::scope_name`](crate::api::core::v1::ScopedResourceSelectorRequirement::scope_name)
    pub enum ResourceQuotaScope {
        /// Match all pod objects where spec.activeDeadlineSeconds \>=0
        Terminating = "Terminating",

        /// Match all pod objects where spec.activeDeadlineSeconds is nil
        NotTerminating = "NotTerminating",

        /// Match all pod objects that have best effort quality of service
        BestEffort = "BestEffort",

        /// Match all pod objects that do not have best effort quality of service
        NotBestEffort = "NotBestEffort",

        /// Match all pod objects that have priority class mentioned
        PriorityClass = "PriorityClass",

        /// Match all pod objects that have cross-namespace pod (anti)affinity mentioned.
        CrossNamespacePodAffinity = "CrossNamespacePodAffinity",

        /// Match all pvc objects that have volume attributes class mentioned.
        VolumeAttributesClass = "VolumeAttributesClass",
    }
}
