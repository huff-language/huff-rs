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
    /// An open brace
    OpenBrace,
    /// A close brace
    CloseBrace,
    /// An open bracket
    OpenBracket,
    /// A close bracket
    CloseBracket,
    /// A comma
    Comma,
    /// A newline
    Newline,
    /// "#define" keyword
    Define,
    /// "takes" keyword
    Takes,
    /// "returns" keyword
    Returns,
    /// "="
    Equal,
    /// Type of function
    FuncType,
    /// "function" keyword
    Function,
    /// "constant" keyword
    Constant,
    /// "FREE_STORAGE_POINTER()" keyword
    FreeStoragePointer,
    /// "macro" keyword
    Macro,
    /// Hex
    Hex,
    /// Opcode
    Opcode,
    /// End Of File
    EOF,
    /// Type of a parameter
    Type,
    /// Huff label (aka PC)
    Label,
    /// Unnamed args [just the types] : list
    Args,
    /// Named args
    NamedArgs,
    /// Body of a macro
    Body,
    /// Import path
    Path,
    /// Statement
    Statement,
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
            TokenKind::Str(str) => str,
            TokenKind::Num(num) => return write!(f, "{}", num),
            TokenKind::Ident(_) => todo!(),
            _ => "oof"
        };

        write!(f, "{}", x)
    }
}
