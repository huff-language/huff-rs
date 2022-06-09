

/// An aliased output location to derive from the cli arguments.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct OutputLocation(pub(crate) String);

impl Default for OutputLocation {
    fn default() -> Self {
        Self("./artifacts/".to_string())
    }
}

/// File Encapsulation
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct FileSource {
    /// File ID
    pub id: Uuid,
    /// File Path
    pub path: String,
    /// File Source
    pub source: Option<String>,
    /// Last File Access Time
    pub access: Option<SystemTime>,
    /// An Ordered List of File Dependencies
    pub dependencies: Option<Vec<FileSource>>,
}
