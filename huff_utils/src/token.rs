use crate::span::Span;
use std::fmt;

type Literal = [u8; 32];

/// A single Token
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Token<'a> {
    /// The kind of token
    pub kind: TokenKind<'a>,
    /// An associated Span
    pub span: Span,
}

impl<'a> Token<'a> {
    /// Public associated function that instantiates a Token.
    pub fn new(kind: TokenKind<'a>, span: Span) -> Self {
        Self { kind, span }
    }
}

/// The kind of token
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind<'a> {
    /// A Comment
    Comment(&'a str),
    /// Division
    /// Lexing done at the comment level due to clash
    Div,
    /// "#define" keyword
    Define,
    /// "#include" keyword
    Include,
    /// "macro" keyword
    Macro,
    /// "function" keyword
    Function,
    /// "constant" keyword
    Constant,
    /// "takes" keyword
    Takes,
    /// "returns" keyword
    Returns,
    /// "FREE_STORAGE_POINTER()" keyword
    FreeStoragePointer,
    /// Equal Sign
    Assign,
    /// An open parenthesis
    OpenParen,
    /// A close parenthesis
    CloseParen,
    /// An open bracket
    OpenBracket,
    /// A close bracket
    CloseBracket,
    /// An open brace
    OpenBrace,
    /// A close brace
    CloseBrace,
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// A comma
    Comma,
    /// Number
    Num(usize),
    /// A Space
    Whitespace,
    /// A string literal
    Str(&'a str),

    // TODO below aren't lexed
    /// An Identifier
    Ident(&'a str),
    /// Hex
    Literal(Literal),
    /// Opcode
    Opcode,
    /// Huff label (aka PC)
    Label(&'a str),

    // TODO: recursive dependency resolution at the lexing level?
    // Import path
    // Path(&'a str),
}

impl<'a> fmt::Display for TokenKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match *self {
            TokenKind::Add => "+",
            TokenKind::Sub => "+",
            TokenKind::Mul => "*",
            TokenKind::Div => "/",
            TokenKind::Whitespace => " ",
            TokenKind::Assign => "=",
            TokenKind::OpenParen => "(",
            TokenKind::CloseParen => ")",
            TokenKind::Comma => ",",
            TokenKind::Str(str) => str,
            TokenKind::Num(num) => return write!(f, "{}", num),
            TokenKind::Ident(_) => todo!(),
            _ => "oof",
        };

        write!(f, "{}", x)
    }
}
