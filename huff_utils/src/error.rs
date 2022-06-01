use std::io::Write;
use crate::{
    report::{Report, Reporter},
    span::{Span, Spanned},
};

/// A Lexing Error
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LexicalError {
    /// The kind of error
    pub kind: LexicalErrorKind,
    /// The span where the error occured
    pub span: Span,
}

impl LexicalError {
    /// Public associated function to instatiate a new LexicalError.
    pub fn new(kind: LexicalErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// A Lexical Error Kind
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LexicalErrorKind {
    /// Unexpected end of file
    UnexpectedEof,
    /// Invalid character
    InvalidCharacter(char),
}

impl Spanned for LexicalError {
    fn span(&self) -> Span {
        self.span
    }
}

impl<W: Write> Report<W> for LexicalError {
    fn report(&self, f: &mut Reporter<'_, W>) -> std::io::Result<()> {
        match self.kind {
            LexicalErrorKind::InvalidCharacter(ch) => write!(f.out, "Invalid character '{}'", ch),
            LexicalErrorKind::UnexpectedEof => write!(f.out, "Found unexpected EOF"),
        }
    }
}
