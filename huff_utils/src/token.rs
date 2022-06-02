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
    /// "#include" keyword
    Include,
    /// "takes" keyword
    Takes,
    /// "returns" keyword
    Returns,
    /// "="
    Equal,
    /// "macro" keyword
    Macro,
    /// "function" keyword
    Function,
    /// "constant" keyword
    Constant,
    /// "FREE_STORAGE_POINTER()" keyword
    FreeStoragePointer,
    /// Hex
    Literal(Literal),
    /// Opcode
    Opcode,
    /// End Of File
    Eof,
    /// Huff label (aka PC)
    Label(&'a str),
    /// Import path
    Path(&'a str),
    /// A Comment
    Comment(&'a str),
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
            _ => "oof",
        };

        write!(f, "{}", x)
    }
}

type FilePath<'a> = &'a str;

///
pub struct Contract<'a> {
    ///
    macros: Vec<MacroDefinition<'a>>,
    ///
    invocations: Vec<MacroInvocation<'a>>,
    ///
    imports: Vec<FilePath<'a>>,
}

///
pub struct MacroDefinition<'a> {
    ///
    pub name: String,
    ///
    pub arguments: Vec<String>,
    ///
    pub statements: Vec<Statement<'a>>,
    ///
    pub takes: usize,
    ///
    pub returns: usize,
}

impl MacroDefinition<'_> {
    fn new<'a>(name: String, arguments: Vec<String>, statements: Vec<Statement<'static>>, takes: usize, returns: usize) -> Self {
        MacroDefinition {
            name,
            arguments,
            statements,
            takes,
            returns,
        }
    }
}

///
pub struct MacroInvocation<'a> {
    ///
    macro_name: String,
    ///
    args: Vec<&'a Literal>,
}

///
pub struct ConstantDefinition<'a> {
    ///
    value: Literal,
    ///
    name: &'a str,
}

///
pub enum Statement<'a> {
    ///
    Literal(Literal),
    ///
    Opcode,
    ///
    MacroInvocation(MacroInvocation<'a>),
}