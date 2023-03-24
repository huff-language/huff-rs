use std::{convert::Infallible, fmt};

use huff_utils::prelude::CompilerError;
use revm::primitives::EVMError;

/// A Runner error
#[derive(Debug)]
pub struct RunnerError(pub String);

/// fmt::Display implementation for `RunnerError`
impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runner Error: {}", self.0)
    }
}

/// Convert a `CompilerError` to a `RunnerError`
impl From<CompilerError> for RunnerError {
    fn from(e: CompilerError) -> Self {
        RunnerError(e.to_string())
    }
}

/// Convert a `EVMError` to a `RunnerError`
impl From<EVMError<Infallible>> for RunnerError {
    fn from(e: EVMError<Infallible>) -> Self {
        RunnerError(format!("{e:?}"))
    }
}
