// Generated from Go type k8s.io/api/batch/v1.ConcurrencyPolicy

crate::string_enum! {
    /// ConcurrencyPolicy describes how the job will be handled.
    /// Only one of the following concurrent policies may be specified.
    /// If none of the following policies is specified, the default one
    /// is AllowConcurrent.
    ///
    /// Used by:
    ///
    /// - [`CronJobSpec::concurrency_policy`](crate::api::batch::v1::CronJobSpec::concurrency_policy)
    pub enum ConcurrencyPolicy {
        /// AllowConcurrent allows CronJobs to run concurrently.
        Allow = "Allow",

        /// ForbidConcurrent forbids concurrent runs, skipping next run if previous
        /// hasn't finished yet.
        Forbid = "Forbid",

        /// ReplaceConcurrent cancels currently running job and replaces it with a new one.
        Replace = "Replace",
    }
}
