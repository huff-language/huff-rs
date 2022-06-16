use crate::{
    files::{Span, Spanned},
    io::UnpackError,
    prelude::parse_extension,
    report::{Report, Reporter},
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
#[derive(Debug, PartialEq, Eq, Clone)]
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
        self.span.clone()
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
pub struct CodegenError {
    /// The kind of code generation error
    pub kind: CodegenErrorKind,
    /// An Optional Span where the error occured
    pub span: Option<Span>,
    /// An Optional Token Kind
    pub token: Option<TokenKind>,
}

impl CodegenError {
    /// Public associated function to instatiate a new CodegenError.
    pub fn new(kind: CodegenErrorKind, span: Option<Span>, token: Option<TokenKind>) -> Self {
        Self { kind, span, token }
    }
}

/// The Code Generation Error Kind
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CodegenErrorKind {
    /// Invalid Operator
    InvalidOperator,
    /// Missing AST
    MissingAst,
    /// AST is missing constructor
    MissingConstructor,
    /// Storage Pointers Not Derived
    StoragePointersNotDerived,
    /// Invalid Macro Body Statement
    InvalidMacroStatement,
    /// The Macro Definition is Missing
    MissingMacroDefinition(String),
    /// Failed to recurse macro
    FailedMacroRecursion,
    /// Missing Constant Definition
    MissingConstantDefinition,
    /// Abi Generation Failure
    AbiGenerationFailure,
    /// An IO Error
    IOError(String),
    /// ArgCall has an unknown type
    UnkownArgcallType,
    /// Missing Macro Invocation
    MissingMacroInvocation(String),
}

impl Spanned for CodegenError {
    fn span(&self) -> Span {
        self.span.clone().unwrap()
    }
}

impl<W: Write> Report<W> for CodegenError {
    fn report(&self, f: &mut Reporter<'_, W>) -> std::io::Result<()> {
        match &self.kind {
            CodegenErrorKind::InvalidOperator => write!(f.out, "Invalid operator!"),
            CodegenErrorKind::MissingAst => write!(f.out, "Codegen is missing an AST!"),
            CodegenErrorKind::MissingConstructor => write!(f.out, "AST missing constructor macro!"),
            CodegenErrorKind::StoragePointersNotDerived => {
                write!(f.out, "Storage pointers not derived for AST!")
            }
            CodegenErrorKind::InvalidMacroStatement => write!(f.out, "Invalid Macro Statement!"),
            CodegenErrorKind::MissingMacroDefinition(str) => {
                write!(f.out, "Missing Macro \"{}\" Definition!", str)
            }
            CodegenErrorKind::FailedMacroRecursion => write!(f.out, "Failed Macro Recursion!"),
            CodegenErrorKind::MissingConstantDefinition => {
                write!(f.out, "Missing Constant Definition!")
            }
            CodegenErrorKind::AbiGenerationFailure => write!(f.out, "Abi generation failure!"),
            CodegenErrorKind::IOError(ioe) => write!(f.out, "IO ERROR: {:?}", ioe),
            CodegenErrorKind::UnkownArgcallType => write!(f.out, "Unknown Argcall Type!"),
            CodegenErrorKind::MissingMacroInvocation(str) => {
                write!(f.out, "Missing Macro \"{}\" Invocation!", str)
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
    CodegenError(CodegenError),
    /// Multiple Failed Compiles
    FailedCompiles(Vec<CompilerError<'a>>),
}

impl<'a> fmt::Display for CompilerError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilerError::LexicalError(le) => match le.kind {
                LexicalErrorKind::UnexpectedEof => {
                    write!(
                        f,
                        "\nError: Unexpected End Of File {}{}\n",
                        le.span.identifier(),
                        le.span.source_seg()
                    )
                }
                LexicalErrorKind::InvalidCharacter(c) => {
                    write!(
                        f,
                        "\nError: Invalid Character: \"{}\" {}{}\n",
                        c,
                        le.span.identifier(),
                        le.span.source_seg()
                    )
                }
                LexicalErrorKind::InvalidArraySize(a) => {
                    write!(
                        f,
                        "\nError: Invalid Array Size: \"{}\" {}{}\n",
                        a,
                        le.span.identifier(),
                        le.span.source_seg()
                    )
                }
                LexicalErrorKind::InvalidPrimitiveType(ty) => {
                    write!(
                        f,
                        "\nError: Invalid Primitive Type: \"{}\" {}{}\n",
                        ty,
                        le.span.identifier(),
                        le.span.source_seg()
                    )
                }
            },
            CompilerError::FileUnpackError(ue) => match ue {
                UnpackError::InvalidDirectory(id) => {
                    write!(f, "\nError: Invalid File Directory {}\n", id)
                }
                UnpackError::UnsupportedExtension(unsupported) => {
                    write!(
                        f,
                        "\nError: Unsupported File Extension \"{}\"\n--> {}\n",
                        parse_extension(unsupported).unwrap_or(""),
                        unsupported
                    )
                }
            },
            CompilerError::ParserError(pe) => write!(f, "ParserError({:?})", pe),
            CompilerError::PathBufRead(os_str) => write!(f, "PathBufRead({:?})", os_str),
            CompilerError::CodegenError(ce) => write!(f, "CodegenError({:?})", ce),
            CompilerError::FailedCompiles(v) => write!(f, "FailedCompiles({})", v.len()),
        }
    }
}
