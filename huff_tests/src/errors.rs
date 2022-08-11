use std::fmt;

/// A Runner error
#[derive(Debug)]
pub struct RunnerError(pub String);

impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runner Error: {}", self.0)
    }
}
