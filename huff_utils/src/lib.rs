//! Utils
//!
//! Refactored utilities commonly used across the huff-rs project.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]

/// Abi Module
pub mod abi;

/// Artifact Module
pub mod artifact;

/// AST Module
pub mod ast;

/// Bytecode Traits Module
pub mod bytecode;

/// Span Module
pub mod span;

/// Token Module
pub mod token;

/// Lexing Error Module
pub mod error;

/// EVM Module
pub mod evm;

/// Files Module
pub mod files;

/// Lexical Reporting Module
pub mod report;

/// IO Module
pub mod io;

/// EVM Types Module
pub mod types;

/// Bytes Util Module
pub mod bytes_util;

/// Prelude wraps common utilities.
pub mod prelude {
    pub use crate::{
        abi::*, artifact::*, ast::*, bytecode::*, bytes_util::*, error::*, io::*, report::*,
        span::*, token::*, files::*,
    };
}
