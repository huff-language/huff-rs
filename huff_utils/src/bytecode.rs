//! Bytecode Traits
//!
//! Abstract translating state into bytecode.

use crate::prelude::Statement;
use std::{
    collections::BTreeMap,
    fmt::{self, Display},
};

/// A string of Bytes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytes(pub String);

/// Intermediate Bytecode Representation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IRByte {
    /// Bytes
    Bytes(Bytes),
    /// Macro Statement to be expanded
    Statement(Statement),
    /// A Constant to be referenced
    Constant(String),
    /// An Arg Call needs to use the calling macro context
    ArgCall(String),
}

/// Full Intermediate Bytecode Representation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IRBytecode(pub Vec<IRByte>);

/// ToIRBytecode
///
/// Converts a stateful object to intermediate bytecode
pub trait ToIRBytecode<E> {
    /// Translates `self` to intermediate bytecode representation
    fn to_irbytecode(&self) -> Result<IRBytecode, E>;
}

/// Full Bytecode
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytecode(pub String);

/// ToBytecode
///
/// Converts a stateful object to bytecode
pub trait ToBytecode<'a, E> {
    /// Translates `self` to a bytecode string
    fn to_bytecode(&self) -> Result<Bytecode, E>;
}

impl From<Vec<Bytes>> for Bytecode {
    fn from(b: Vec<Bytes>) -> Self {
        Bytecode(b.iter().fold("".to_string(), |acc, b| format!("{}{}", acc, b.0)))
    }
}

/// Result type for [huff_codegen](../../huff_codegen)'s
/// [`recurse_bytecode`](../../huff_codegen/src/lib.rs#recurse_bytecode)
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BytecodeRes {
    /// Resulting bytes
    pub bytes: Vec<Bytes>,
    /// Jump Tables
    pub jump_tables: Vec<JumpTable>,
    /// Jump Indices
    pub jump_indices: JumpIndices,
    /// Unmatched Jumps
    pub unmatched_jumps: Jumps,
}

impl Display for BytecodeRes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"BytecodeRes(
            bytes: [{}],
            jump_tables: {:?},
            jump_indices: {:?},
            unmatched_jumps: {:?}
        )"#,
            self.bytes.iter().fold("".to_string(), |acc, b| format!("{}{}", acc, b.0)),
            self.jump_tables,
            self.jump_indices,
            self.unmatched_jumps
        )
    }
}

/// A Jump
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Jump {
    /// Jump's Label
    pub label: String,
    /// Index of jump within bytecode
    pub bytecode_index: usize,
}

/// Type for a vec of `Jump`s
pub type Jumps = Vec<Jump>;

/// Type to map `Jump`s to
pub type JumpIndices = BTreeMap<Jump, usize>;

/// Type for a map of bytecode indexes to `Jumps`. Represents a Jump Table.
pub type JumpTable = BTreeMap<usize, Jumps>;
