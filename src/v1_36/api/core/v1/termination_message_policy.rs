// Generated from Go type k8s.io/api/core/v1.TerminationMessagePolicy

crate::string_enum! {
    /// TerminationMessagePolicy describes how termination messages are retrieved from a container.
    ///
    /// Used by:
    ///
    /// - [`Container::termination_message_policy`](crate::api::core::v1::Container::termination_message_policy)
    pub enum TerminationMessagePolicy {
        /// TerminationMessageReadFile is the default behavior and will set the container status message to
        /// the contents of the container's terminationMessagePath when the container exits.
        File = "File",

        /// TerminationMessageFallbackToLogsOnError will read the most recent contents of the container logs
        /// for the container status message when the container exits with an error and the
        /// terminationMessagePath has no contents.
        FallbackToLogsOnError = "FallbackToLogsOnError",
    }
}
