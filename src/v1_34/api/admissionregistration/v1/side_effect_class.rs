// Generated from Go type k8s.io/api/admissionregistration/v1.SideEffectClass

crate::string_enum! {
    /// SideEffectClass specifies the types of side effects a webhook may have.
    ///
    /// Used by:
    ///
    /// - [`ValidatingWebhook::side_effects`](crate::api::admissionregistration::v1::ValidatingWebhook::side_effects)
    /// - [`MutatingWebhook::side_effects`](crate::api::admissionregistration::v1::MutatingWebhook::side_effects)
    pub enum SideEffectClass {
        /// SideEffectClassUnknown means that no information is known about the side effects of calling the webhook.
        /// If a request with the dry-run attribute would trigger a call to this webhook, the request will instead fail.
        Unknown = "Unknown",

        /// SideEffectClassNone means that calling the webhook will have no side effects.
        None = "None",

        /// SideEffectClassSome means that calling the webhook will possibly have side effects.
        /// If a request with the dry-run attribute would trigger a call to this webhook, the request will instead fail.
        Some = "Some",

        /// SideEffectClassNoneOnDryRun means that calling the webhook will possibly have side effects, but if the
        /// request being reviewed has the dry-run attribute, the side effects will be suppressed.
        NoneOnDryRun = "NoneOnDryRun",
    }
}
