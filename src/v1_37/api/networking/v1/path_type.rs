// Generated from Go type k8s.io/api/networking/v1.PathType

crate::string_enum! {
    /// PathType represents the type of path referred to by a HTTPIngressPath.
    ///
    /// Used by:
    ///
    /// - [`HTTPIngressPath::path_type`](crate::api::networking::v1::HTTPIngressPath::path_type)
    pub enum PathType {
        /// PathTypeExact matches the URL path exactly and with case sensitivity.
        Exact = "Exact",

        /// PathTypePrefix matches based on a URL path prefix split by '/'. Matching
        /// is case sensitive and done on a path element by element basis. A path
        /// element refers to the list of labels in the path split by the '/'
        /// separator. A request is a match for path p if every p is an element-wise
        /// prefix of p of the request path. Note that if the last element of the
        /// path is a substring of the last element in request path, it is not a
        /// match (e.g. /foo/bar matches /foo/bar/baz, but does not match
        /// /foo/barbaz). If multiple matching paths exist in an Ingress spec, the
        /// longest matching path is given priority.
        /// Examples:
        /// - /foo/bar does not match requests to /foo/barbaz
        /// - /foo/bar matches request to /foo/bar and /foo/bar/baz
        /// - /foo and /foo/ both match requests to /foo and /foo/. If both paths are
        ///   present in an Ingress spec, the longest matching path (/foo/) is given
        ///   priority.
        Prefix = "Prefix",

        /// PathTypeImplementationSpecific matching is up to the IngressClass.
        /// Implementations can treat this as a separate PathType or treat it
        /// identically to Prefix or Exact path types.
        ImplementationSpecific = "ImplementationSpecific",
    }
}
