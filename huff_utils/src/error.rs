use crate::{
    report::{Report, Reporter},
    span::{Span, Spanned},
};
use std::io::Write;

/// A Lexing Error
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LexicalError<'a> {
    /// The kind of error
    pub kind: LexicalErrorKind<'a>,
    /// The span where the error occured
    pub span: Span,
}

impl<'a> LexicalError<'a> {
    /// Public associated function to instatiate a new LexicalError.
    pub fn new(kind: LexicalErrorKind<'a>, span: Span) -> Self {
        Self { kind, span }
    }
}

/// A Lexical Error Kind
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LexicalErrorKind<'a> {
    /// Unexpected end of file
    UnexpectedEof,
    /// Invalid character
    InvalidCharacter(char),
    /// Invalid Array Size
    /// String param expected to be usize parsable
    InvalidArraySize(&'a str),
}

impl<'a> Spanned for LexicalError<'a> {
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, W: Write> Report<W> for LexicalError<'a> {
    fn report(&self, f: &mut Reporter<'_, W>) -> std::io::Result<()> {
        match self.kind {
            LexicalErrorKind::InvalidCharacter(ch) => write!(f.out, "Invalid character '{}'", ch),
            LexicalErrorKind::UnexpectedEof => write!(f.out, "Found unexpected EOF"),
            LexicalErrorKind::InvalidArraySize(str) => {
                write!(f.out, "Invalid array size: '{}'", str)
            }
        }
    }
}

/// A Code Generation Error
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CodegenError {
    /// The kind of code generation error
    pub kind: CodegenErrorKind,
    /// The span where the error occured
    pub span: Span,
}

impl CodegenError {
    /// Public associated function to instatiate a new CodegenError.
    pub fn new(kind: CodegenErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// The Code Generation Error Kind
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CodegenErrorKind {
    /// Invalid Operator
    InvalidOperator,
}

impl Spanned for CodegenError {
    fn span(&self) -> Span {
        self.span
    }
}

impl<W: Write> Report<W> for CodegenError {
    fn report(&self, f: &mut Reporter<'_, W>) -> std::io::Result<()> {
        match self.kind {
            // CodegenErrorKind::ExpectedIntExpr => write!(f.out, "Expected integer expression"),
            // CodegenErrorKind::ExpectedIdent => write!(f.out, "Expected identifier"),
            CodegenErrorKind::InvalidOperator => write!(f.out, "Invalid operator"),
        }
    }
}
