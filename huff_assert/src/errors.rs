use huff_tests::types::TestStatus;
use std::fmt::{format, Display, Formatter};

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Returns,
    Takes,
    Value,
    Amount,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let m = match self {
            ErrorKind::Returns => format!("mismatching return"),
            ErrorKind::Takes => format!("mismatching takes"),
            ErrorKind::Value => format!("wrong assertion value"),
            ErrorKind::Amount => format!("wrong return"),
        };

        write!(f, "{}", m)
    }
}

#[derive(Debug, Clone)]
pub struct AssertError {
    pub kind: ErrorKind,
    pub expected: String,
    pub got: String,
}

impl Display for AssertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: expected `{:?}` got `{:?}`", self.kind, self.expected, self.got)
    }
}

/// A stack assertion result
#[derive(Debug, Clone)]
pub struct AssertResult {
    pub name: String,
    pub status: TestStatus,
    pub errors: Vec<AssertError>,
}
