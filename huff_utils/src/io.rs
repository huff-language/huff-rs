use std::{ffi::OsStr, path::Path};

/// Returns a file extension from a path as a string.
pub fn parse_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

/// Unpacking errors
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum UnpackError {
    /// The file extension is not supported.
    UnsupportedExtension(String),
    /// Failed to read directory
    InvalidDirectory(String),
    /// Missing File
    MissingFile(String),
}

/// Unpacks huff files into a vec of strings.
pub fn unpack_files(path: &str) -> Result<Vec<String>, UnpackError> {
    // If the path is a file, return a vec of the file
    match parse_extension(path) {
        Some(extension) => {
            if extension == "huff" {
                return Ok(vec![path.to_string()])
            }
            Err(UnpackError::UnsupportedExtension(path.to_string()))
        }
        None => {
            // We have a directory, try to extract huff files and parse
            match std::fs::read_dir(path) {
                Ok(files) => {
                    let input_files: Vec<String> =
                        files.map(|x| x.unwrap().path().to_str().unwrap().to_string()).collect();
                    let filtered: Vec<String> = input_files
                        .iter()
                        .filter(|&f| Path::new(&f).extension().unwrap_or_default().eq("huff"))
                        .cloned()
                        .collect();
                    Ok(filtered)
                }
                Err(e) => {
                    tracing::error!(target: "io", "ERROR READING DIRECTORY {}: {:?}", path, e);
                    Err(UnpackError::InvalidDirectory(path.to_string()))
                }
            }
        }
    }
}
