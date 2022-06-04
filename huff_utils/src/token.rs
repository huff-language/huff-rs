use crate::{evm::Opcode, span::Span};
use std::{fmt, fmt::Write};

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
    /// EOF Token
    Eof,
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
    /// "event" keyword
    Event,
    /// "constant" keyword
    Constant,
    /// "takes" keyword
    Takes,
    /// "returns" keyword
    Returns,
    /// "FREE_STORAGE_POINTER()" keyword
    FreeStoragePointer,
    /// An Identifier
    Ident(&'a str),
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
    /// Hex
    Literal(Literal),
    /// Opcode
    Opcode(Opcode),
    /// Huff label (aka PC)
    Label(&'a str),
    // TODO: recursive dependency resolution at the lexing level?
    // Import path
    // Path(&'a str),
}

impl<'a> fmt::Display for TokenKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match *self {
            TokenKind::Eof => "EOF",
            TokenKind::Comment(s) => return write!(f, "Comment({})", s),
            TokenKind::Div => "/",
            TokenKind::Define => "#define",
            TokenKind::Include => "#include",
            TokenKind::Macro => "macro",
            TokenKind::Function => "function",
            TokenKind::Event => "event",
            TokenKind::Constant => "constant",
            TokenKind::Takes => "takes",
            TokenKind::Returns => "returns",
            TokenKind::FreeStoragePointer => "FREE_STORAGE_POINTER()",
            TokenKind::Ident(s) => return write!(f, "{}", s),
            TokenKind::Assign => "=",
            TokenKind::OpenParen => "(",
            TokenKind::CloseParen => ")",
            TokenKind::OpenBracket => "[",
            TokenKind::CloseBracket => "]",
            TokenKind::OpenBrace => "{",
            TokenKind::CloseBrace => "}",
            TokenKind::Add => "+",
            TokenKind::Sub => "+",
            TokenKind::Mul => "*",
            TokenKind::Comma => ",",
            TokenKind::Num(num) => return write!(f, "{}", num),
            TokenKind::Whitespace => " ",
            TokenKind::Str(str) => str,

            // TODO these aren't lexed yet
            TokenKind::Literal(l) => {
                let mut s = String::new();
                for b in l.iter() {
                    let _ = write!(&mut s, "{:02x}", b);
                }
                return write!(f, "{}", s)
            }
            TokenKind::Opcode(o) => return write!(f, "{}", o),
            TokenKind::Label(s) => return write!(f, "{}", s),
        };

        write!(f, "{}", x)
    }
}
