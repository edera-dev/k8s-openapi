// Generated from Go type k8s.io/api/core/v1.PodPhase

crate::string_enum! {
    /// PodPhase is a label for the condition of a pod at the current time.
    ///
    /// Used by:
    ///
    /// - [`PodStatus::phase`](crate::api::core::v1::PodStatus::phase)
    pub enum PodPhase {
        /// PodPending means the pod has been accepted by the system, but one or more of the containers
        /// has not been started. This includes time before being bound to a node, as well as time spent
        /// pulling images onto the host.
        Pending = "Pending",

        /// PodRunning means the pod has been bound to a node and all of the containers have been started.
        /// At least one container is still running or is in the process of being restarted.
        Running = "Running",

        /// PodSucceeded means that all containers in the pod have voluntarily terminated
        /// with a container exit code of 0, and the system is not going to restart any of these containers.
        Succeeded = "Succeeded",

        /// PodFailed means that all containers in the pod have terminated, and at least one container has
        /// terminated in a failure (exited with a non-zero exit code or was stopped by the system).
        Failed = "Failed",

        /// PodUnknown means that for some reason the state of the pod could not be obtained, typically due
        /// to an error in communicating with the host of the pod.
        /// Deprecated: It isn't being set since 2015 (74da3b14b0c0f658b3bb8d2def5094686d0e9095)
        Unknown = "Unknown",
    }
}
