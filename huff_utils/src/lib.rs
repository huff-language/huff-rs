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

/// Solidity Interface Generator
pub mod sol_interface;

/// File Provider Module
pub mod file_provider;

/// Time Module
pub mod time;

/// Wasm Module
pub mod wasm;

/// EVM Version Module
pub mod evm_version;

/// Prelude wraps common utilities.
pub mod prelude {
    pub use crate::{
        abi::*, artifact::*, ast::*, bytecode::*, bytes_util::*, error::*, evm::*, evm_version::*,
        files::*, io::*, report::*, sol_interface::*, token::*, types::*,
    };
}
