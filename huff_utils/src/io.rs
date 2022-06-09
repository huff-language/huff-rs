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
            if extension == "huff" {
                return Ok(vec![path.to_string()])
            }
            Err(UnpackError::UnsupportedExtension)
        }
        None => {
            // We have a directory, try to extract huff files and parse
            match std::fs::read_dir(path) {
                Ok(files) => {
                    let input_files: Vec<String> =
                        files.map(|x| x.unwrap().path().to_str().unwrap().to_string()).collect();
                    let filtered: Vec<String> = input_files
                        .iter()
                        .filter(|&f| Path::new(&f).extension().unwrap().eq("huff"))
                        .cloned()
                        .collect();
                    Ok(filtered)
                }
                Err(_) => Err(UnpackError::InvalidDirectory),
            }
        }
    }
}
