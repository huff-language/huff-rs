use std::fmt::{self};

use strum_macros::Display;

use crate::span::Span;

/// A single Token
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token<'a> {
    /// The kind of token
    pub kind: TokenKind<'a>,
    /// An associated Span
    pub span: Span,
}

/// The kind of token
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind<'a> {
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// Division
    Div,
    /// Semicolon
    Semi,
    /// Equal Sign
    Assign,
    /// Number
    Num(usize),
    /// A string literal
    Str(&'a str),
    /// An Identifier
    Ident(&'a str),
    /// A Space
    Whitespace,
    /// An open parenthesis
    OpenParen,
    /// A close parenthesis
    CloseParen,
    /// A comma
    Comma,

    // --- Huff Specific Compatibility ---
    /// A comment with its comments encapsulated for traceability
    Comment(String),
    /// A Definition
    Definition(Definition),
}

// TODO: Can we make this a hash of the name?
// TODO: Are there macro conflicts?
/// A Macro Identifier
pub type MacroIdentifier = String;

/// A Definition
#[derive(Debug, PartialEq, Eq, Clone, Display)]
pub enum Definition {
    /// A Macro
    Macro(MacroIdentifier),
    /// An imported file
    Import(String),
}

impl<'a> fmt::Display for TokenKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match self {
            TokenKind::Comment(str) => return write!(f, "Comment({})", str),
            TokenKind::Definition(str) => return write!(f, "{:?} Definition", str),
            TokenKind::Add => "+",
            TokenKind::Sub => "+",
            TokenKind::Mul => "*",
            TokenKind::Div => "/",
            TokenKind::Whitespace => " ",
            TokenKind::Semi => ";",
            TokenKind::Assign => "=",
            TokenKind::OpenParen => "(",
            TokenKind::CloseParen => ")",
            TokenKind::Comma => ",",
            TokenKind::Str(str) => str,
            TokenKind::Num(num) => return write!(f, "{}", num),
            TokenKind::Ident(_) => todo!(),
        };

        write!(f, "{}", x)
    }
}
