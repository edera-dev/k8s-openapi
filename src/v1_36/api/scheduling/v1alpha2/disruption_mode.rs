// Generated from Go type k8s.io/api/scheduling/v1alpha2.DisruptionMode

crate::string_enum! {
    /// DisruptionMode describes the mode in which a PodGroup can be disrupted (e.g. preempted).
    ///
    /// Used by:
    ///
    /// - [`PodGroupTemplate::disruption_mode`](crate::api::scheduling::v1alpha2::PodGroupTemplate::disruption_mode)
    /// - [`PodGroupSpec::disruption_mode`](crate::api::scheduling::v1alpha2::PodGroupSpec::disruption_mode)
    pub enum DisruptionMode {
        /// DisruptionModePod means that individual pods can be disrupted or preempted independently.
        /// It doesn't depend on exact set of pods currently running in this PodGroup.
        Pod = "Pod",

        /// DisruptionModePodGroup means that the whole PodGroup needs to be disrupted
        /// or preempted together.
        PodGroup = "PodGroup",
    }
}
