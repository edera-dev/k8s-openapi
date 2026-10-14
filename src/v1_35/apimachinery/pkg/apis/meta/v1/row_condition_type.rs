// Generated from Go type k8s.io/apimachinery/pkg/apis/meta/v1.RowConditionType

crate::string_enum! {
    /// Used by:
    ///
    /// - [`TableRowCondition::type_`](crate::apimachinery::pkg::apis::meta::v1::TableRowCondition::type_)
    pub enum RowConditionType {
        /// RowCompleted means the underlying resource has reached completion and may be given less
        /// visual priority than other resources.
        Completed = "Completed",
    }
}
