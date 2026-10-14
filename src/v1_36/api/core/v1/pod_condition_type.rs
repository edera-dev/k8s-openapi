// Generated from Go type k8s.io/api/core/v1.PodConditionType

crate::string_enum! {
    /// PodConditionType is a valid value for PodCondition.Type
    ///
    /// Used by:
    ///
    /// - [`PodFailurePolicyOnPodConditionsPattern::type_`](crate::api::batch::v1::PodFailurePolicyOnPodConditionsPattern::type_)
    /// - [`PodCondition::type_`](crate::api::core::v1::PodCondition::type_)
    /// - [`PodReadinessGate::condition_type`](crate::api::core::v1::PodReadinessGate::condition_type)
    pub enum PodConditionType {
        /// ContainersReady indicates whether all containers in the pod are ready.
        ContainersReady = "ContainersReady",

        /// PodInitialized means that all init containers in the pod have started successfully.
        Initialized = "Initialized",

        /// PodReady means the pod is able to service requests and should be added to the
        /// load balancing pools of all matching services.
        Ready = "Ready",

        /// PodScheduled represents status of the scheduling process for this pod.
        PodScheduled = "PodScheduled",

        /// DisruptionTarget indicates the pod is about to be terminated due to a
        /// disruption (such as preemption, eviction API or garbage-collection).
        DisruptionTarget = "DisruptionTarget",

        /// PodReadyToStartContainers pod sandbox is successfully configured and
        /// the pod is ready to launch containers.
        PodReadyToStartContainers = "PodReadyToStartContainers",

        /// PodResizePending indicates that the pod has been resized, but kubelet has not
        /// yet allocated the resources. If both PodResizePending and PodResizeInProgress
        /// are set, it means that a new resize was requested in the middle of a previous
        /// pod resize that is still in progress.
        PodResizePending = "PodResizePending",

        /// PodResizeInProgress indicates that a resize is in progress, and is present whenever
        /// the Kubelet has allocated resources for the resize, but has not yet actuated all of
        /// the required changes.
        /// If both PodResizePending and PodResizeInProgress are set, it means that a new resize was
        /// requested in the middle of a previous pod resize that is still in progress.
        PodResizeInProgress = "PodResizeInProgress",

        /// AllContainersRestarting indicates that all containers of the pod is being restarted.
        AllContainersRestarting = "AllContainersRestarting",
    }
}
