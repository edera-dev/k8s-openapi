// Generated from Go type k8s.io/api/policy/v1.UnhealthyPodEvictionPolicyType

crate::string_enum! {
    /// UnhealthyPodEvictionPolicyType defines the criteria for when unhealthy pods
    /// should be considered for eviction.
    ///
    /// Used by:
    ///
    /// - [`PodDisruptionBudgetSpec::unhealthy_pod_eviction_policy`](crate::api::policy::v1::PodDisruptionBudgetSpec::unhealthy_pod_eviction_policy)
    pub enum UnhealthyPodEvictionPolicyType {
        /// IfHealthyBudget policy means that running pods (status.phase="Running"),
        /// but not yet healthy can be evicted only if the guarded application is not
        /// disrupted (status.currentHealthy is at least equal to status.desiredHealthy).
        /// Healthy pods will be subject to the PDB for eviction.
        IfHealthyBudget = "IfHealthyBudget",

        /// AlwaysAllow policy means that all running pods (status.phase="Running"),
        /// but not yet healthy are considered disrupted and can be evicted regardless
        /// of whether the criteria in a PDB is met. This means perspective running
        /// pods of a disrupted application might not get a chance to become healthy.
        /// Healthy pods will be subject to the PDB for eviction.
        AlwaysAllow = "AlwaysAllow",
    }
}
