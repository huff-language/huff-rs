use crate::{
    report::{Report, Reporter},
    span::{Span, Spanned},
};
use std::io::Write;

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

/// A Parser Error
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum ParserError {
    /// A general syntax error that accepts a message
    SyntaxError(&'static str),
    /// Unexpected type
    UnexpectedType,
    /// Invalid definition
    InvalidDefinition,
    /// Invalid constant value
    InvalidConstantValue,
    /// Invalid name (macro, event, function, constant)
    InvalidName,
    /// Invalid arguments
    InvalidArgs,
    /// Invalid macro call arguments
    InvalidMacroArgs,
    /// Invalid return arguments
    InvalidReturnArgs,
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
