//! ## Artifact
//!
//! The artifacts generated from codegen.

use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

pub use crate::abi::Abi;
use crate::prelude::FileSource;

/// A Codegen Artifact
#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Artifact {
    /// The file source
    pub file: FileSource,
    /// The deployed bytecode
    pub bytecode: String,
    /// The runtime bytecode
    pub runtime: String,
    /// The abi
    pub abi: Option<Abi>,
}

impl Artifact {
    /// Exports an artifact to a json file
    pub fn export(&self, out: String) -> std::result::Result<(), std::io::Error> {
        let serialized_artifact = serde_json::to_string(self)?;
        let file_path = Path::new(&out);
        if let Some(p) = file_path.parent() {
            fs::create_dir_all(p)?
        }
        fs::write(file_path, serialized_artifact)
    }
}