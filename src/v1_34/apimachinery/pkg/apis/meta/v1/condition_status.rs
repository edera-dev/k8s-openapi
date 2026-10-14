// Generated from Go type k8s.io/apimachinery/pkg/apis/meta/v1.ConditionStatus

crate::string_enum! {
    /// Used by:
    ///
    /// - [`TableRowCondition::status`](crate::apimachinery::pkg::apis::meta::v1::TableRowCondition::status)
    /// - [`Condition::status`](crate::apimachinery::pkg::apis::meta::v1::Condition::status)
    pub enum ConditionStatus {
        True = "True",

        False = "False",

        Unknown = "Unknown",
    }
}
