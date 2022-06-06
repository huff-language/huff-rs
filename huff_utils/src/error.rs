use crate::{
    report::{Report, Reporter},
    span::{Span, Spanned}, token::TokenKind,
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
pub struct CodegenError<'a> {
    /// The kind of code generation error
    pub kind: CodegenErrorKind,
    /// An Optional Span where the error occured
    pub span: Option<Span>,
    /// An Optional Token Kind
    pub token: Option<TokenKind<'a>>
}

impl<'a> CodegenError<'a> {
    /// Public associated function to instatiate a new CodegenError.
    pub fn new(kind: CodegenErrorKind, span: Option<Span>, token: Option<TokenKind<'a>>) -> Self {
        Self { kind, span, token }
    }
}

/// The Code Generation Error Kind
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CodegenErrorKind {
    /// Invalid Operator
    InvalidOperator,
    /// Missing AST
    MissingAst,
}

impl<'a> Spanned for CodegenError<'a> {
    fn span(&self) -> Span {
        self.span.unwrap()
    }
}

impl<'a, W: Write> Report<W> for CodegenError<'a> {
    fn report(&self, f: &mut Reporter<'_, W>) -> std::io::Result<()> {
        match self.kind {
            // CodegenErrorKind::ExpectedIntExpr => write!(f.out, "Expected integer expression"),
            // CodegenErrorKind::ExpectedIdent => write!(f.out, "Expected identifier"),
            CodegenErrorKind::InvalidOperator => write!(f.out, "Invalid operator"),
            CodegenErrorKind::MissingAst => write!(f.out, "Codegen is missing an AST"),
        }
    }
}
