// Generated from Go type k8s.io/api/core/v1.PullPolicy

crate::string_enum! {
    /// PullPolicy describes a policy for if/when to pull a container image
    ///
    /// Used by:
    ///
    /// - [`Container::image_pull_policy`](crate::api::core::v1::Container::image_pull_policy)
    /// - [`ImageVolumeSource::pull_policy`](crate::api::core::v1::ImageVolumeSource::pull_policy)
    pub enum PullPolicy {
        /// PullAlways means that kubelet always attempts to pull the latest image. Container will fail If the pull fails.
        Always = "Always",

        /// PullNever means that kubelet never pulls an image, but only uses a local image. Container will fail if the image isn't present
        Never = "Never",

        /// PullIfNotPresent means that kubelet pulls if the image isn't present on disk. Container will fail if the image isn't present and the pull fails.
        IfNotPresent = "IfNotPresent",
    }
}
