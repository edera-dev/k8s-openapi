// Generated from Go type k8s.io/api/autoscaling/v1.MetricSourceType

crate::string_enum! {
    /// MetricSourceType indicates the type of metric.
    pub enum MetricSourceType {
        /// ObjectMetricSourceType is a metric describing a kubernetes object
        /// (for example, hits-per-second on an Ingress object).
        Object = "Object",

        /// PodsMetricSourceType is a metric describing each pod in the current scale
        /// target (for example, transactions-processed-per-second).  The values
        /// will be averaged together before being compared to the target value.
        Pods = "Pods",

        /// ResourceMetricSourceType is a resource metric known to Kubernetes, as
        /// specified in requests and limits, describing each pod in the current
        /// scale target (e.g. CPU or memory).  Such metrics are built in to
        /// Kubernetes, and have special scaling options on top of those available
        /// to normal per-pod metrics (the "pods" source).
        Resource = "Resource",

        /// ContainerResourceMetricSourceType is a resource metric known to Kubernetes, as
        /// specified in requests and limits, describing a single container in each pod in the current
        /// scale target (e.g. CPU or memory).  Such metrics are built in to
        /// Kubernetes, and have special scaling options on top of those available
        /// to normal per-pod metrics (the "pods" source).
        ContainerResource = "ContainerResource",

        /// ExternalMetricSourceType is a global metric that is not associated
        /// with any Kubernetes object. It allows autoscaling based on information
        /// coming from components running outside of cluster
        /// (for example length of queue in cloud messaging service, or
        /// QPS from loadbalancer running outside of cluster).
        External = "External",
    }
}
