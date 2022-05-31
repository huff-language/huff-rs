
use std::fmt;

use crate::span::Span;

/// A single Token
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token<'a> {
  /// The kind of token
  pub kind: TokenKind<'a>,
  /// An associated Span
  pub span: Span,
}

/// The kind of token
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    /// An Identifier
    Ident(&'a str),
    /// A string literal
    Str(&'a str),
    /// A numeric literal
    Num(usize),
    /// A 32-byte EVM slot
    Literal(&'a [u8;32]),
    /// A Space
    Whitespace,
    /// An open parenthesis
    OpenParen,
    /// A close parenthesis
    CloseParen,
    /// A comma
    Comma,
    /// Beginning of a macro, constant, function or event definition.
    Define,
    /// A macro
    Macro,
    /// Takes
    Takes(usize),
    /// Returns
    Returns(usize)
}

impl<'a> fmt::Display for TokenKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match *self {
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
            TokenKind::Macro => "macro",
            TokenKind::Takes(num) => todo!(),
            TokenKind::Returns(num) => todo!(),
            TokenKind::Define => "#define",
            TokenKind::Ident(_) => todo!(),
            TokenKind::Num(num) => todo!(),
            TokenKind::Str(str) => todo!(),
            TokenKind::Literal(raw) => todo!()
        };

        write!(f, "{}", x)
    }
}
