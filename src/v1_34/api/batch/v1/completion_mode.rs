// Generated from Go type k8s.io/api/batch/v1.CompletionMode

crate::string_enum! {
    /// CompletionMode specifies how Pod completions of a Job are tracked.
    ///
    /// Used by:
    ///
    /// - [`JobSpec::completion_mode`](crate::api::batch::v1::JobSpec::completion_mode)
    pub enum CompletionMode {
        /// NonIndexedCompletion is a Job completion mode. In this mode, the Job is
        /// considered complete when there have been .spec.completions
        /// successfully completed Pods. Pod completions are homologous to each other.
        NonIndexed = "NonIndexed",

        /// IndexedCompletion is a Job completion mode. In this mode, the Pods of a
        /// Job get an associated completion index from 0 to (.spec.completions - 1).
        /// The Job is  considered complete when a Pod completes for each completion
        /// index.
        Indexed = "Indexed",
    }
}
