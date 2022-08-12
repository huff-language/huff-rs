use std::fmt;

use huff_utils::prelude::CompilerError;

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
impl From<CompilerError<'_>> for RunnerError {
    fn from(e: CompilerError) -> Self {
        RunnerError(e.to_string())
    }
}
