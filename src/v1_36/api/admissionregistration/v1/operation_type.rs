// Generated from Go type k8s.io/api/admissionregistration/v1.OperationType

crate::string_enum! {
    /// OperationType specifies an operation for a request.
    ///
    /// Used by:
    ///
    /// - [`RuleWithOperations::operations`](crate::api::admissionregistration::v1::RuleWithOperations::operations)
    pub enum OperationType {
        OperationAll = "*",

        CREATE = "CREATE",

        UPDATE = "UPDATE",

        DELETE = "DELETE",

        CONNECT = "CONNECT",
    }
}
