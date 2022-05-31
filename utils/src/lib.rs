//! Utils
//!
//! Refactored utilities commonly used across the rhuff project.

#![deny(missing_docs)]

/// Span module
pub mod span;

/// Prelude wraps common utilities.
pub mod prelude {
    pub use crate::span::*;
}