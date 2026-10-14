// Generated from Go type k8s.io/api/resource/v1beta2.SkipNodeOperation

crate::string_enum! {
    /// Used by:
    ///
    /// - [`ResourceSliceSpec::skip_node_operations`](crate::api::resource::v1beta2::ResourceSliceSpec::skip_node_operations)
    /// - [`DeviceRequestAllocationResult::skip_node_operations`](crate::api::resource::v1beta2::DeviceRequestAllocationResult::skip_node_operations)
    pub enum SkipNodeOperation {
        /// SkipNodeOperationNodePrepareResources indicates that NodePrepareResources gRPC calls are skipped.
        NodePrepareResources = "NodePrepareResources",

        /// SkipNodeOperationNodeUnprepareResources indicates that NodeUnprepareResources gRPC calls are skipped.
        NodeUnprepareResources = "NodeUnprepareResources",

        /// SkipNodeOperationAll indicates that all node-local resource operations are skipped.
        All = "*",
    }
}
