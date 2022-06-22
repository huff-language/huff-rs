
/// Constant Bytecode Generation Module
pub mod constants;

/// Statement Bytecode Generation Module
pub mod statements;


/// Prelude wraps common utilities.
pub mod prelude {
    pub use super::{
      constants::*,
      statements::*
    };
}