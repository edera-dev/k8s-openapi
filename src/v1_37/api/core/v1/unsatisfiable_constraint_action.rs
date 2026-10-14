// Generated from Go type k8s.io/api/core/v1.UnsatisfiableConstraintAction

crate::string_enum! {
    /// Used by:
    ///
    /// - [`TopologySpreadConstraint::when_unsatisfiable`](crate::api::core::v1::TopologySpreadConstraint::when_unsatisfiable)
    pub enum UnsatisfiableConstraintAction {
        /// DoNotSchedule instructs the scheduler not to schedule the pod
        /// when constraints are not satisfied.
        DoNotSchedule = "DoNotSchedule",

        /// ScheduleAnyway instructs the scheduler to schedule the pod
        /// even if constraints are not satisfied.
        ScheduleAnyway = "ScheduleAnyway",
    }
}
