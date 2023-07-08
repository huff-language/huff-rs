//! Bytecode Traits
//!
//! Abstract translating state into bytecode.

use crate::{
    evm_version::EVMVersion,
    prelude::{AstSpan, Statement, TableDefinition},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{self, Display},
};

/// A string of Bytes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytes(pub String);

/// Intermediate Bytecode Representation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IRBytes<'a> {
    /// The type of IRByte
    pub ty: IRByteType,
    /// The Span of the IRBytes
    pub span: &'a AstSpan,
}

/// IRBytes Type
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IRByteType {
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
pub struct IRBytecode<'a>(pub Vec<IRBytes<'a>>);

/// ToIRBytecode
///
/// Converts a stateful object to intermediate bytecode
pub trait ToIRBytecode<E> {
    /// Translates `self` to intermediate bytecode representation
    fn to_irbytecode(&self, evm_version: &EVMVersion) -> Result<IRBytecode, E>;
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
        Bytecode(b.iter().fold("".to_string(), |acc, b| format!("{acc}{}", b.0)))
    }
}

/// Result type for [huff_codegen](../../huff_codegen)'s
/// [`recurse_bytecode`](../../huff_codegen/src/lib.rs#recurse_bytecode)
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BytecodeRes {
    /// Resulting bytes
    pub bytes: Vec<(usize, Bytes)>,
    /// Jump Indices
    pub label_indices: LabelIndices,
    /// Unmatched Jumps
    pub unmatched_jumps: Jumps,
    /// Table Instances
    pub table_instances: Jumps,
    /// Utilized Tables
    pub utilized_tables: Vec<TableDefinition>,
}

impl Display for BytecodeRes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"BytecodeRes(
            bytes: [{}],
            label_indices: {:?},
            unmatched_jumps: {:?}
            table_instances: {:?}
        )"#,
            self.bytes.iter().fold("".to_string(), |acc, b| format!("{acc}{}", b.0)),
            self.label_indices,
            self.unmatched_jumps,
            self.table_instances
        )
    }
}

/// A Jump
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Jump {
    /// Jump's Label
    pub label: String,
    /// Index of jump within bytecode
    pub bytecode_index: usize,
    /// The Jump Span
    pub span: AstSpan,
}

/// Type for a vec of `Jump`s
pub type Jumps = Vec<Jump>;

/// Type to map `Jump` labels to their bytecode indices
pub type LabelIndices = BTreeMap<String, usize>;

/// Typw to map circular_codesize labels to their bytecode indices
pub type CircularCodeSizeIndices = BTreeSet<(String, usize)>;

/// Type for a map of bytecode indexes to `Jumps`. Represents a Jump Table.
pub type JumpTable = BTreeMap<usize, Jumps>;
