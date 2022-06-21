use crate::{
    io::UnpackError,
    report::{Report, Reporter},
    span::{Span, Spanned},
    token::TokenKind,
};
use std::{ffi::OsString, fmt, io::Write};

/// A Parser Error
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum ParserError {
    /// A general syntax error that accepts a message
    SyntaxError(String),
    /// Unexpected type
    UnexpectedType,
    /// Invalid definition
    InvalidDefinition,
    /// Invalid constant value
    InvalidConstantValue,
    /// Invalid constant
    InvalidConstant,
    /// Invalid name (macro, event, function, constant)
    InvalidName,
    /// Invalid arguments
    InvalidArgs,
    /// Invalid macro call arguments
    InvalidMacroArgs,
    /// Invalid return arguments
    InvalidReturnArgs,
    /// Invalid import path
    InvalidImportPath,
}

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
    /// Invalid Primitive EVM Type
    InvalidPrimitiveType(&'a str),
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
            LexicalErrorKind::InvalidPrimitiveType(str) => {
                write!(f.out, "Invalid Primitive EVM Type '{}'", str)
            }
        }
    }
}

/// A Code Generation Error
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CodegenError<'a> {
    /// The kind of code generation error
    pub kind: CodegenErrorKind,
    /// An Optional Span where the error occured
    pub span: Option<Span>,
    /// An Optional Token Kind
    pub token: Option<TokenKind<'a>>,
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
    /// AST is missing constructor
    MissingConstructor,
    /// Invalid Macro Body Statement
    InvalidMacroStatement,
    /// The Macro Definition is Missing
    MissingMacroDefinition,
    /// Failed to recurse macro
    FailedMacroRecursion,
    /// Missing Constant Definition
    MissingConstantDefinition,
}

impl Spanned for CodegenError<'_> {
    fn span(&self) -> Span {
        self.span.unwrap()
    }
}

impl<W: Write> Report<W> for CodegenError<'_> {
    fn report(&self, f: &mut Reporter<'_, W>) -> std::io::Result<()> {
        match self.kind {
            CodegenErrorKind::InvalidOperator => write!(f.out, "Invalid operator!"),
            CodegenErrorKind::MissingAst => write!(f.out, "Codegen is missing an AST!"),
            CodegenErrorKind::MissingConstructor => write!(f.out, "AST missing constructor macro!"),
            CodegenErrorKind::InvalidMacroStatement => write!(f.out, "Invalid Macro Statement!"),
            CodegenErrorKind::MissingMacroDefinition => write!(f.out, "Missing Macro Definition!"),
            CodegenErrorKind::FailedMacroRecursion => write!(f.out, "Failed Macro Recursion!"),
            CodegenErrorKind::MissingConstantDefinition => {
                write!(f.out, "Missing Constant Definition!")
            }
        }
    }
}

/// CompilerError
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilerError<'a> {
    /// Failed to Lex Source
    LexicalError(LexicalError<'a>),
    /// File unpacking error
    FileUnpackError(UnpackError),
    /// Parsing Error
    ParserError(ParserError),
    /// Reading PathBuf Failed
    PathBufRead(OsString),
    /// Bytecode Generation Error
    CodegenError(CodegenError<'a>),
}

impl<'a> fmt::Display for CompilerError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilerError::LexicalError(le) => write!(f, "LexicalError({:?})", le),
            CompilerError::FileUnpackError(ue) => write!(f, "FileUnpackError({:?})", ue),
            CompilerError::ParserError(pe) => write!(f, "ParserError({:?})", pe),
            CompilerError::PathBufRead(os_str) => write!(f, "PathBufRead({:?})", os_str),
            CompilerError::CodegenError(ce) => write!(f, "CodegenError({:?})", ce),
        }
    }
}
