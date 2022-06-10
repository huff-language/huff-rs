use std::{path::PathBuf, time::SystemTime};
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

    /// Derives a File Path's directory
    pub fn derive_dir(path: &str) -> Option<String> {
        let path = PathBuf::from(path);
        match path.parent() {
            Some(p) => p.to_str().map(String::from),
            None => None,
        }
    }

    /// Localizes a file path, if path is relative
    pub fn localize_file(parent: &str, child: &str) -> Option<String> {
        let mut prefix = match FileSource::derive_dir(parent) {
            Some(p) => {
                if p.is_empty() {
                    String::from(".")
                } else {
                    p
                }
            }
            None => String::from("."),
        };
        if child.starts_with("../") {
            let mut res_str = child.to_string();
            while res_str.starts_with("../") {
                let path = PathBuf::from(prefix.clone());
                match path.parent() {
                    Some(p) => match p.to_str().map(String::from) {
                        Some(pref) => {
                            if pref.is_empty() || prefix.ends_with("..") {
                                if prefix.is_empty() || prefix == "." {
                                    prefix = "..".to_string();
                                } else {
                                    prefix = format!("../{}", prefix);
                                }
                            } else {
                                prefix = pref
                            }
                        }
                        None => {
                            tracing::warn!("Failed to convert path to string");
                            return None
                        }
                    },
                    None => {
                        tracing::warn!("Failed to find parent for path: {:?}", path);
                        return None
                    }
                }
                res_str = res_str.replacen("../", "", 1);
            }
            Some(format!("{}/{}", prefix, res_str))
        } else if child.starts_with("./") {
            Some(child.replacen("./", &format!("{}/", prefix), 1))
        } else if child.starts_with('/') {
            Some(child.to_string())
        } else {
            Some(format!("{}/{}", prefix, child))
        }
    }
}
