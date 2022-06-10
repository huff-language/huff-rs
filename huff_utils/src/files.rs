use std::time::SystemTime;
use uuid::Uuid;

/// An aliased output location to derive from the cli arguments.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct OutputLocation(pub String);

impl Default for OutputLocation {
    fn default() -> Self {
        Self("./artifacts/".to_string())
    }
}

/// File Encapsulation
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

impl FileSource {
    /// Generates a fully flattened source code for the given `FileSource` and all its dependencies
    ///
    /// ### Examples
    ///
    /// Let's say you have a file, `a.txt` with two dependencies, `b.txt` and `c.txt`,
    /// `fully_flatten()` will generate a source code string with the contents of `b.txt` and
    /// `c.txt` appended to the end of the contents of `a.txt`.
    pub fn fully_flatten(&self) -> String {
        // First grab the parent file source
        let mut full_source = if let Some(s) = &self.source { s.clone() } else { "".to_string() };

        // Then recursively grab source code for dependencies
        match &self.dependencies {
            Some(vfs) => {
                for fs in vfs {
                    full_source.push_str(&fs.fully_flatten())
                }
            }
            None => {}
        }

        // Return the full source
        full_source
    }
}
