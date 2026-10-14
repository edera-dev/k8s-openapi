// Generated from Go type k8s.io/api/admissionregistration/v1alpha1.PatchType

crate::string_enum! {
    /// PatchType specifies the type of patch operation for a mutation.
    ///
    /// Used by:
    ///
    /// - [`Mutation::patch_type`](crate::api::admissionregistration::v1alpha1::Mutation::patch_type)
    pub enum PatchType {
        /// ApplyConfiguration indicates that the mutation is using apply configuration to mutate the object.
        ApplyConfiguration = "ApplyConfiguration",

        /// JSONPatch indicates that the object is mutated through JSON Patch.
        JSONPatch = "JSONPatch",
    }
}
