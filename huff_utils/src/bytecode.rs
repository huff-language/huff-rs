//! Bytecode Traits
//!
//! Abstract translating state into bytecode.

use crate::prelude::Statement;

/// A Single Byte
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Byte(pub String);

/// Intermediate Bytecode Representation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IRByte<'a> {
    /// Bytes
    Byte(Byte),
    /// Macro Statement to be expanded
    Statement(Statement<'a>),
    /// A Constant to be referenced
    Constant(&'a str),
    /// An Arg Call needs to use the calling macro context
    ArgCall(&'a str),
}

/// Full Intermediate Bytecode Representation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct IRBytecode<'a>(pub Vec<IRByte<'a>>);

/// ToIRBytecode
///
/// Converts a stateful object to intermediate bytecode
pub trait ToIRBytecode<'a, E> {
    /// Translates `self` to intermediate bytecode representation
    fn to_irbytecode(&self) -> Result<IRBytecode<'a>, E>;
}

/// Full Bytecode
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytecode<'a>(pub &'a str);

/// ToBytecode
///
/// Converts a stateful object to bytecode
pub trait ToBytecode<'a, E> {
    /// Translates `self` to a bytecode string
    fn to_bytecode(&self) -> Result<Bytecode<'a>, E>;
}
