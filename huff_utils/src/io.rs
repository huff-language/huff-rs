use std::{ffi::OsStr, path::Path};

/// Returns a file extension from a path as a string.
pub fn parse_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

/// Unpacking errors
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum UnpackError {
    /// The file extension is not supported.
    UnsupportedExtension,
    /// Failed to read directory
    InvalidDirectory,
}

/// Unpacks huff files into a vec of strings.
pub fn unpack_files(path: &str) -> Result<Vec<String>, UnpackError> {
    // If the path is a file, return a vec of the file
    match parse_extension(path) {
        Some(extension) => {
            tracing::info!(target: "io", "FOUND HUFF FILE: {}", extension);
            if extension == "huff" {
                return Ok(vec![path.to_string()])
            }
            Err(UnpackError::UnsupportedExtension)
        }
        None => {
            // We have a directory, try to extract huff files and parse
            tracing::info!(target: "io", "READING HUFF FILES IN: {}", path);
            match std::fs::read_dir(path) {
                Ok(files) => {
                    tracing::info!(target: "io", "FOUND FILES: {:?}", files);
                    let input_files: Vec<String> =
                        files.map(|x| x.unwrap().path().to_str().unwrap().to_string()).collect();
                    tracing::info!(target: "io", "COLLECTED INPUT FILES: {:?}", input_files);
                    let filtered: Vec<String> = input_files
                        .iter()
                        .filter(|&f| Path::new(&f).extension().unwrap_or_default().eq("huff"))
                        .cloned()
                        .collect();
                    tracing::info!(target: "io", "FILTERED INPUT FILES: {:?}", filtered);
                    Ok(filtered)
                }
                Err(e) => {
                    tracing::error!(target: "io", "ERROR READING DIRECTORY {}: {:?}", path, e);
                    Err(UnpackError::InvalidDirectory)
                }
            }
        }
    }
}
