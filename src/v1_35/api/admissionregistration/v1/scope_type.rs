// Generated from Go type k8s.io/api/admissionregistration/v1.ScopeType

crate::string_enum! {
    /// ScopeType specifies a scope for a Rule.
    pub enum ScopeType {
        /// ClusterScope means that scope is limited to cluster-scoped objects.
        /// Namespace objects are cluster-scoped.
        Cluster = "Cluster",

        /// NamespacedScope means that scope is limited to namespaced objects.
        Namespaced = "Namespaced",

        /// AllScopes means that all scopes are included.
        AllScopes = "*",
    }
}
