// Generated from Go type k8s.io/apimachinery/pkg/apis/meta/v1.IncludeObjectPolicy

crate::string_enum! {
    /// IncludeObjectPolicy controls which portion of the object is returned with a Table.
    pub enum IncludeObjectPolicy {
        /// IncludeNone returns no object.
        None = "None",

        /// IncludeMetadata serializes the object containing only its metadata field.
        Metadata = "Metadata",

        /// IncludeObject contains the full object.
        Object = "Object",
    }
}
