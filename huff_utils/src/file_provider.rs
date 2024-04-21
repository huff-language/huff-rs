use crate::{
    error::CompilerError,
    files::FileSource,
    io::{unpack_files, UnpackError},
    time,
};
use std::{
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};
use uuid::Uuid;

/// Provides functions to supply file contents by paths.
pub trait FileProvider<'a>: Send + Sync + Debug {
    /// Returns a FileSource containing the file contents referred to by the supplied path.
    fn read_file(&self, pb: PathBuf) -> Result<Arc<FileSource>, CompilerError>;

    /// Takes a list of strings and returns a transformed list PathBufs.
    fn transform_paths(&self, sources: &[String]) -> Result<Vec<PathBuf>, CompilerError>;
}

/// A FileReader that reads files from the filesystem.
#[derive(Debug)]
pub struct FileSystemFileProvider {}

impl Default for FileSystemFileProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystemFileProvider {
    /// Creates a new instance of a FileSystemFileReader.
    pub fn new() -> Self {
        FileSystemFileProvider {}
    }
}

impl<'a> FileProvider<'a> for FileSystemFileProvider {
    fn read_file(&self, pb: PathBuf) -> Result<Arc<FileSource>, CompilerError> {
        let file_loc = String::from(pb.to_string_lossy());
        match std::fs::read_to_string(&file_loc) {
            Ok(source) => Ok(Arc::new(FileSource {
                id: Uuid::new_v4(),
                path: file_loc,
                source: Some(source),
                access: Some(time::get_current_time()),
                dependencies: None,
            })),
            Err(_) => {
                tracing::error!(target: "core", "FILE READ FAILED: \"{}\"!", file_loc);
                Err(CompilerError::FileUnpackError(UnpackError::MissingFile(file_loc)))
            }
        }
    }

    fn transform_paths(&self, sources: &[String]) -> Result<Vec<PathBuf>, CompilerError> {
        let mut paths = vec![];
        for f in sources {
            // If the file is huff, use the path, otherwise unpack
            let ext = Path::new(&f).extension().unwrap_or_default();
            if ext.eq("huff") {
                paths.push(Path::new(&f).to_path_buf())
            } else {
                // Otherwise, override the source files and use all files in the provided dir
                match unpack_files(f) {
                    Ok(files) => {
                        files.iter().for_each(|fil| paths.push(Path::new(&fil).to_path_buf()))
                    }
                    Err(e) => {
                        tracing::error!(target: "core", "ERROR UNPACKING FILE: {:?}", e);
                        return Err(CompilerError::FileUnpackError(e));
                    }
                }
            }
        }
        Ok(paths)
    }
}

/// A FileReader which reads files from memory via a supplied HashMap.
#[derive(Debug)]
pub struct InMemoryFileProvider {
    /// A mapping of paths to file contents.
    pub sources: Arc<HashMap<String, String>>,
}

impl InMemoryFileProvider {
    /// Creates a new instance of a MemoryFileReader.
    pub fn new(mut sources: HashMap<String, String>) -> Self {
        // Localize all path keys to ./
        let mut sources_localized: HashMap<String, String> = HashMap::new();
        sources.drain().for_each(|(path, source)| {
            sources_localized.insert(strip_path_prefix(path.as_str()).to_owned(), source);
        });
        InMemoryFileProvider { sources: Arc::new(sources_localized) }
    }
}

impl<'a> FileProvider<'a> for InMemoryFileProvider {
    fn read_file(&self, pb: PathBuf) -> Result<Arc<FileSource>, CompilerError> {
        let path = pb.to_str().unwrap_or_default();
        let localized = strip_path_prefix(path);
        match self.sources.get(localized) {
            Some(source) => Ok(Arc::new(FileSource {
                id: Uuid::new_v4(),
                path: path.to_string(),
                source: Some(source.to_string()),
                access: Some(time::get_current_time()),
                dependencies: None,
            })),
            None => {
                tracing::error!(target: "core", "FILE READ FAILED: \"{}\"!", path);
                Err(CompilerError::FileUnpackError(UnpackError::MissingFile(path.to_string())))
            }
        }
    }

    fn transform_paths(&self, sources: &[String]) -> Result<Vec<PathBuf>, CompilerError> {
        let mut paths = vec![];
        for f in sources {
            // If the file is huff, use the path, otherwise ignore
            let ext = Path::new(&f).extension().unwrap_or_default();
            if ext.eq("huff") {
                let localized = strip_path_prefix(f);
                paths.push(Path::new(&localized).to_path_buf())
            }
        }
        Ok(paths)
    }
}

fn strip_path_prefix(path: &str) -> &str {
    path.strip_prefix("./").unwrap_or(path)
}
