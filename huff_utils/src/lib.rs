//! Utils
//!
//! Refactored utilities commonly used across the huff-rs project.

#![deny(missing_docs)]

/// Abi Module
pub mod abi;

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

/// Bytes Util
pub mod bytes_util;

/// Prelude wraps common utilities.
pub mod prelude {
    pub use crate::{abi::*, ast::*, bytes_util::*, error::*, io::*, report::*, span::*, token::*};
}
