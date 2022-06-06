//! ## Artifact
//!
//! The artifacts generated from codegen.

use serde::{Deserialize, Serialize};

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
