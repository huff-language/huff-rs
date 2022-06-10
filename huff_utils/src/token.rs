use crate::{
    evm::{Opcode, OPCODES_MAP},
    span::Span,
};
use logos::{Lexer, Logos};
use std::fmt;

/// A token
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

fn to_opcode(lex: &mut Lexer<TokenKind>) -> Option<Opcode> {
    OPCODES_MAP.get(lex.slice()).cloned()
}

fn to_string(lex: &mut Lexer<TokenKind>) -> Option<String> {
    lex.slice().parse().ok()
}

fn to_string_literal(lex: &mut Lexer<TokenKind>) -> Option<String> {
    let ch = &['\'', '\"'];
    let s = lex.slice().strip_prefix(ch).unwrap().strip_suffix(ch).unwrap();
    Some(s.to_string())
}

fn to_num(lex: &mut Lexer<TokenKind>) -> Option<usize> {
    lex.slice().parse::<usize>().ok()
}

/// The kind of token
#[derive(Logos, Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    /// "#define" keyword
    #[token("#define")]
    Define,
    /// "#include" keyword
    #[token("#include")]
    Include,
    /// "macro" keyword
    #[token("macro")]
    Macro,
    /// "function" keyword
    #[token("function")]
    Function,
    /// "constant" keyword
    #[token("constant")]
    Constant,
    /// "takes" keyword
    #[token("takes")]
    Takes,
    /// "returns" keyword
    #[token("returns")]
    Returns,
    /// "FREE_STORAGE_POINTER()" keyword
    #[token(r"FREE_STORAGE_POINTER()")]
    FreeStoragePointer,
    /// Equal Sign
    #[token("=")]
    Assign,
    /// An open parenthesis
    #[token("(")]
    OpenParen,
    /// A close parenthesis
    #[token(")")]
    CloseParen,
    /// An open bracket
    #[token("[")]
    OpenBracket,
    /// A close bracket
    #[token("]")]
    CloseBracket,
    /// An open brace
    #[token("{")]
    OpenBrace,
    /// A close brace
    #[token("}")]
    CloseBrace,
    /// Addition
    #[token("+")]
    Add,
    /// Subtraction
    #[token("-")]
    Sub,
    /// Multiplication
    #[token("*")]
    Mul,
    /// Division
    #[token(r"/")]
    Div,
    /// A comma
    #[token(",")]
    Comma,
    /// Number
    #[regex(r"[0-9]+", to_num)]
    Num(usize),
    /// String literal
    #[regex(r#""([^"\\]|\\.)*""#, to_string_literal)]
    #[regex(r#"'([^'\\]|\\.)*'"#, to_string_literal)]
    Str(String),
    /// Hex literal
    #[regex(r"0[xX][a-fA-F0-9]+", to_string)]
    Hex(String),
    /// Opcodes
    #[token("lt", to_opcode)]
    #[token("gt", to_opcode)]
    #[token("slt", to_opcode)]
    #[token("sgt", to_opcode)]
    #[token("eq", to_opcode)]
    #[token("iszero", to_opcode)]
    #[token("and", to_opcode)]
    #[token("origin", to_opcode)]
    #[token("or", to_opcode)]
    #[token("xor", to_opcode)]
    #[token("not", to_opcode)]
    #[token("sha3", to_opcode)]
    #[token("address", to_opcode)]
    #[token("balance", to_opcode)]
    #[token("caller", to_opcode)]
    #[token("callvalue", to_opcode)]
    #[token("calldataload", to_opcode)]
    #[token("calldatasize", to_opcode)]
    #[token("calldatacopy", to_opcode)]
    #[token("codesize", to_opcode)]
    #[token("codecopy", to_opcode)]
    #[token("blockhash", to_opcode)]
    #[token("coinbase", to_opcode)]
    #[token("timestamp", to_opcode)]
    #[token("number", to_opcode)]
    #[token("difficulty", to_opcode)]
    #[token("gaslimit", to_opcode)]
    #[token("chainid", to_opcode)]
    #[token("selfbalance", to_opcode)]
    #[token("pop", to_opcode)]
    #[token("mload", to_opcode)]
    #[token("mstore8", to_opcode)]
    #[token("mstore", to_opcode)]
    #[token("sload", to_opcode)]
    #[token("sstore", to_opcode)]
    #[token("jumpdest", to_opcode)]
    #[token("jumpi", to_opcode)]
    #[token("jump", to_opcode)]
    #[token("pc", to_opcode)]
    #[token("msize", to_opcode)]
    #[token("stop", to_opcode)]
    #[token("addmod", to_opcode)]
    #[token("add", to_opcode)]
    #[token("mulmod", to_opcode)]
    #[token("mul", to_opcode)]
    #[token("sub", to_opcode)]
    #[token("div", to_opcode)]
    #[token("sdiv", to_opcode)]
    #[token("mod", to_opcode)]
    #[token("smod", to_opcode)]
    #[token("exp", to_opcode)]
    #[token("signextend", to_opcode)]
    #[token("byte", to_opcode)]
    #[token("shl", to_opcode)]
    #[token("shr", to_opcode)]
    #[token("sar", to_opcode)]
    #[token("gasprice", to_opcode)]
    #[token("extcodesize", to_opcode)]
    #[token("extcodecopy", to_opcode)]
    #[token("returndatasize", to_opcode)]
    #[token("returndatacopy", to_opcode)]
    #[token("extcodehash", to_opcode)]
    #[token("gas", to_opcode)]
    #[regex("log[0-4]", to_opcode)] // LOG0-LOG4
    #[token("create2", to_opcode)]
    #[token("create", to_opcode)]
    #[token("callcode", to_opcode)]
    #[token("call", to_opcode)]
    #[token("return", to_opcode)]
    #[token("delegatecall", to_opcode)]
    #[token("staticcall", to_opcode)]
    #[token("revert", to_opcode)]
    #[token("invalid", to_opcode)]
    #[token("selfdestruct", to_opcode)]
    #[regex("push([1-9]|[1-2][0-9]|3[0-2])", to_opcode)] // PUSH1-PUSH32
    #[regex("swap([1-9]|1[0-6])", to_opcode)] // SWAP1-SWAP16
    #[regex("dup([1-9]|1[0-6])", to_opcode)] // DUP1-DUP16
    Opcode(Opcode),
    /// Label (aka PC)
    // #[regex(r"\w+:", |lex| lex.slice().parse())]
    // Label(String),
    /// A Comment
    #[regex(r"//.*", to_string)] // single line comment
    #[regex("/\\*[^*]*\\*+(?:[^/*][^*]*\\*+)*/", to_string)] // multi line comment
    Comment(String),
    /// Identifier
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", to_string)]
    Ident(String),
    /// Error
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)] // Whitespace
    Error,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match self {
            TokenKind::Comment(s) => return write!(f, "Comment({})", s),
            TokenKind::Div => "/",
            TokenKind::Define => "#define",
            TokenKind::Include => "#include",
            TokenKind::Macro => "macro",
            TokenKind::Function => "function",
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
            TokenKind::Str(value) => return write!(f, "{}", value),
            TokenKind::Hex(value) => return write!(f, "{}", value),
            TokenKind::Opcode(o) => return write!(f, "{}", o),
            TokenKind::Error => "<error>",
        };

        write!(f, "{}", x)
    }
}
