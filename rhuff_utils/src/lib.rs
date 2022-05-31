//! Utils
//!
//! Refactored utilities commonly used across the rhuff project.

#![deny(missing_docs)]

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

/// Prelude wraps common utilities.
pub mod prelude {
    pub use crate::{error::*, io::*, report::*, span::*, token::*};
}
