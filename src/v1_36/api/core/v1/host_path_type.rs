// Generated from Go type k8s.io/api/core/v1.HostPathType

crate::string_enum! {
    /// Used by:
    ///
    /// - [`HostPathVolumeSource::type_`](crate::api::core::v1::HostPathVolumeSource::type_)
    pub enum HostPathType {
        /// For backwards compatible, leave it empty if unset
        Unset = "",

        /// If nothing exists at the given path, an empty directory will be created there
        /// as needed with file mode 0755, having the same group and ownership with Kubelet.
        DirectoryOrCreate = "DirectoryOrCreate",

        /// A directory must exist at the given path
        Directory = "Directory",

        /// If nothing exists at the given path, an empty file will be created there
        /// as needed with file mode 0644, having the same group and ownership with Kubelet.
        FileOrCreate = "FileOrCreate",

        /// A file must exist at the given path
        File = "File",

        /// A UNIX socket must exist at the given path
        Socket = "Socket",

        /// A character device must exist at the given path
        CharDevice = "CharDevice",

        /// A block device must exist at the given path
        BlockDevice = "BlockDevice",
    }
}
