//! ## Artifact
//!
//! The artifacts generated from codegen.

use serde::{Deserialize, Serialize};
use std::fs;

pub use crate::abi::Abi;

/// A Codegen Artifact
#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Artifact {
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
        fs::write(out, serialized_artifact)
    }
}
