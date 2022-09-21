use huff_tests::types::TestStatus;
use huff_utils::ast::AstSpan;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
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
    pub spans: Option<AstSpan>,
}

impl Display for AssertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: expected {} got {}", self.kind, self.expected, self.got)
    }
}

/// A stack assertion result
#[derive(Debug, Clone)]
pub struct AssertResult {
    pub name: String,
    pub status: TestStatus,
    pub errors: Vec<AssertError>,
}

#[derive(Debug, Clone)]
pub struct PrettyError(pub AssertError);

impl Display for PrettyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let err = format!("{}", &self.0);
        let err = match &self.0.spans {
            Some(spans) => spans.error(Some(&err)),
            None => err,
        };
        write!(f, "{}", err)
    }
}
