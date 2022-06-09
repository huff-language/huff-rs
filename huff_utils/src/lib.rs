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

/// Span Module
pub mod span;

/// Token Module
pub mod token;

/// Lexing Error Module
pub mod error;

/// Lexical Reporting Module
pub mod report;

/// IO Module
pub mod io;

/// EVM Module
pub mod evm;

/// EVM Types Module
pub mod types;

/// Bytes Util
pub mod bytes_util;

/// Prelude wraps common utilities.
pub mod prelude {
    pub use crate::{
        abi::*, artifact::*, ast::*, bytecode::*, bytes_util::*, error::*, io::*, report::*,
        span::*, token::*,
    };
}