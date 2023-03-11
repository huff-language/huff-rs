use crate::{evm::Opcode, files::Span, types::PrimitiveEVMType};
use std::{fmt, fmt::Write};

type Literal = [u8; 32];

/// A single Token
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    /// The kind of token
    pub kind: TokenKind,
    /// An associated Span
    pub span: Span,
}

impl Token {
    /// Public associated function that instantiates a Token.
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// The kind of token
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TokenKind {
    /// EOF Token
    Eof,
    /// A Comment
    Comment(String),
    /// Division
    /// Lexing done at the comment level due to clash
    Div,
    /// "#define" keyword
    Define,
    /// "#include" keyword
    Include,
    /// "macro" keyword
    Macro,
    /// "fn" keyword
    Fn,
    /// "test" keyword
    Test,
    /// "function" keyword
    Function,
    /// "event" keyword
    Event,
    /// "constant" keyword
    Constant,
    /// "error" keyword
    Error,
    /// "takes" keyword
    Takes,
    /// "returns" keyword
    Returns,
    /// "view" keyword
    View,
    /// "pure" keyword
    Pure,
    /// "payable" keyword
    Payable,
    /// "nonpayable" keyword
    NonPayable,
    /// "indexed" keyword
    Indexed,
    /// "FREE_STORAGE_POINTER()" keyword
    FreeStoragePointer,
    /// An Identifier
    Ident(String),
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
    /// A Less-Than Angle Bracket
    LeftAngle,
    /// A Greater-Than Angle Bracket
    RightAngle,
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// A comma
    Comma,
    /// A Colon
    Colon,
    /// A pound
    Pound,
    /// Number
    Num(usize),
    /// A Space
    Whitespace,
    /// A string literal
    Str(String),
    /// Hex
    Literal(Literal),
    /// Opcode
    Opcode(Opcode),
    /// Huff label (aka PC)
    Label(String),
    // TODO: recursive dependency resolution at the lexing level?
    // Import path
    // Path(String),
    /// EVM Type
    PrimitiveType(PrimitiveEVMType),
    /// Array of EVM Types
    /// uint256[5][2][3] => ArrayType(PrimitiveEVMType::Uint(256), [5, 2, 3])
    ArrayType(PrimitiveEVMType, Vec<usize>),
    /// A Jump Table
    JumpTable,
    /// A Packed Jump Table
    JumpTablePacked,
    /// A Code Table
    CodeTable,
    /// A builtin function (__codesize, __tablesize, __tablestart)
    BuiltinFunction(String),
    /// Calldata Data Location
    Calldata,
    /// Memory Data Location
    Memory,
    /// Storage Data Location
    Storage,
}

impl TokenKind {
    /// Transform a single char TokenKind into a Token given a single position
    pub fn into_single_span(self, position: u32) -> Token {
        self.into_span(position, position)
    }

    /// Transform a TokenKind into a Token given a start and end position
    pub fn into_span(self, start: u32, end: u32) -> Token {
        Token { kind: self, span: Span { start: start as usize, end: end as usize, file: None } }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match self {
            TokenKind::Eof => "EOF",
            TokenKind::Comment(s) => return write!(f, "Comment({s})"),
            TokenKind::Div => "/",
            TokenKind::Define => "#define",
            TokenKind::Include => "#include",
            TokenKind::Macro => "macro",
            TokenKind::Fn => "fn",
            TokenKind::Test => "test",
            TokenKind::Function => "function",
            TokenKind::Event => "event",
            TokenKind::Constant => "constant",
            TokenKind::Error => "error",
            TokenKind::View => "view",
            TokenKind::Pure => "pure",
            TokenKind::Payable => "payable",
            TokenKind::NonPayable => "nonpayable",
            TokenKind::Indexed => "indexed",
            TokenKind::Takes => "takes",
            TokenKind::Returns => "returns",
            TokenKind::FreeStoragePointer => "FREE_STORAGE_POINTER()",
            TokenKind::Ident(s) => return write!(f, "{s}"),
            TokenKind::Assign => "=",
            TokenKind::OpenParen => "(",
            TokenKind::CloseParen => ")",
            TokenKind::OpenBracket => "[",
            TokenKind::CloseBracket => "]",
            TokenKind::OpenBrace => "{",
            TokenKind::CloseBrace => "}",
            TokenKind::LeftAngle => "<",
            TokenKind::RightAngle => ">",
            TokenKind::Add => "+",
            TokenKind::Sub => "-",
            TokenKind::Mul => "*",
            TokenKind::Colon => ":",
            TokenKind::Comma => ",",
            TokenKind::Pound => "#",
            TokenKind::Num(num) => return write!(f, "{num}"),
            TokenKind::Whitespace => " ",
            TokenKind::Str(str) => str,
            TokenKind::Literal(l) => {
                let mut s = String::new();
                for b in l.iter() {
                    let _ = write!(&mut s, "{b:02x}");
                }
                return write!(f, "{s}")
            }
            TokenKind::Opcode(o) => return write!(f, "{o}"),
            TokenKind::Label(s) => return write!(f, "{s}"),
            TokenKind::PrimitiveType(pt) => return write!(f, "{pt}"),
            TokenKind::ArrayType(pt, size_vec) => {
                let mut s = String::new();
                for size in size_vec {
                    let brackets = if size > &0 { format!("[{size}]") } else { "[]".to_string() };
                    s.push_str(&brackets);
                }
                return write!(f, "{pt}{s}")
            }
            TokenKind::JumpTable => "jumptable",
            TokenKind::JumpTablePacked => "jumptable__packed",
            TokenKind::CodeTable => "table",
            TokenKind::BuiltinFunction(s) => return write!(f, "BuiltinFunction({s})"),
            TokenKind::Calldata => return write!(f, "calldata"),
            TokenKind::Memory => return write!(f, "memory"),
            TokenKind::Storage => return write!(f, "storage"),
        };

        write!(f, "{x}")
    }
}
