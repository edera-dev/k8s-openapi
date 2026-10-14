// Generated from Go type k8s.io/api/core/v1.NodePhase

crate::string_enum! {
    /// Used by:
    ///
    /// - [`NodeStatus::phase`](crate::api::core::v1::NodeStatus::phase)
    pub enum NodePhase {
        /// NodePending means the node has been created/added by the system, but not configured.
        Pending = "Pending",

        /// NodeRunning means the node has been configured and has Kubernetes components running.
        Running = "Running",

        /// NodeTerminated means the node has been removed from the cluster.
        Terminated = "Terminated",
    }
}
