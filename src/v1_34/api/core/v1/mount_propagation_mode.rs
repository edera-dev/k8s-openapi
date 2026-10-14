// Generated from Go type k8s.io/api/core/v1.MountPropagationMode

crate::string_enum! {
    /// MountPropagationMode describes mount propagation.
    ///
    /// Used by:
    ///
    /// - [`VolumeMount::mount_propagation`](crate::api::core::v1::VolumeMount::mount_propagation)
    pub enum MountPropagationMode {
        /// MountPropagationNone means that the volume in a container will
        /// not receive new mounts from the host or other containers, and filesystems
        /// mounted inside the container won't be propagated to the host or other
        /// containers.
        /// Note that this mode corresponds to "private" in Linux terminology.
        None = "None",

        /// MountPropagationHostToContainer means that the volume in a container will
        /// receive new mounts from the host or other containers, but filesystems
        /// mounted inside the container won't be propagated to the host or other
        /// containers.
        /// Note that this mode is recursively applied to all mounts in the volume
        /// ("rslave" in Linux terminology).
        HostToContainer = "HostToContainer",

        /// MountPropagationBidirectional means that the volume in a container will
        /// receive new mounts from the host or other containers, and its own mounts
        /// will be propagated from the container to the host or other containers.
        /// Note that this mode is recursively applied to all mounts in the volume
        /// ("rshared" in Linux terminology).
        Bidirectional = "Bidirectional",
    }
}
