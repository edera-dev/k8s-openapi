// Generated from Go type k8s.io/api/core/v1.IPFamilyPolicy

crate::string_enum! {
    /// IPFamilyPolicy represents the dual-stack-ness requested or required by a Service
    ///
    /// Used by:
    ///
    /// - [`ServiceSpec::ip_family_policy`](crate::api::core::v1::ServiceSpec::ip_family_policy)
    pub enum IPFamilyPolicy {
        /// IPFamilyPolicySingleStack indicates that this service is required to have a single IPFamily.
        /// The IPFamily assigned is based on the default IPFamily used by the cluster
        /// or as identified by service.spec.ipFamilies field
        SingleStack = "SingleStack",

        /// IPFamilyPolicyPreferDualStack indicates that this service prefers dual-stack when
        /// the cluster is configured for dual-stack. If the cluster is not configured
        /// for dual-stack the service will be assigned a single IPFamily. If the IPFamily is not
        /// set in service.spec.ipFamilies then the service will be assigned the default IPFamily
        /// configured on the cluster
        PreferDualStack = "PreferDualStack",

        /// IPFamilyPolicyRequireDualStack indicates that this service requires dual-stack. Using
        /// IPFamilyPolicyRequireDualStack on a single stack cluster will result in validation errors. The
        /// IPFamilies (and their order) assigned  to this service is based on service.spec.ipFamilies. If
        /// service.spec.ipFamilies was not provided then it will be assigned according to how they are
        /// configured on the cluster. If service.spec.ipFamilies has only one entry then the alternative
        /// IPFamily will be added by apiserver
        RequireDualStack = "RequireDualStack",
    }
}
