use crate::span::Span;
use logos::Logos;
use std::fmt;

/// A token
#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Clone, Default)]
pub struct TokenExtras {
    /// Whether the current context is a scope i.e. lexing b/w `{` and `}`
    pub in_scope: bool,
}

/// The kind of token
#[derive(Logos, Debug, PartialEq, Eq, Clone)]
#[logos(extras = TokenExtras)]
pub enum TokenKind<'a> {
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
    #[token("{", |lex| lex.extras.in_scope = true)]
    OpenBrace,
    /// A close brace
    #[token("}", |lex| lex.extras.in_scope = false)]
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
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Num(usize),
    /// String literal
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        // Strip surrounding `"`
        let s: &str = lex.slice();
        &s[1..s.len()-1]
    })]
    #[regex(r#"'([^'\\]|\\.)*'"#, |lex| {
        // Strip surrounding `'`
        let s: &str = lex.slice();
        &s[1..s.len()-1]
    })]
    Str(&'a str),
    /// Hex literal
    #[regex(r"0[xX][a-fA-F0-9]+")]
    Hex(&'a str),
    /// Opcodes
    #[regex(r"stop|addmod|add|mulmod|mul|sub|div|sdiv|mod|smod|exp|signextend")]
    #[regex(r"lt|gt|slt|sgt|eq|iszero|and|or|xor|not|byte|shl|shr|sar")]
    #[regex(r"address|balance|origin|caller|callvalue|calldataload|calldatasize|calldatacopy|codesize|codecopy|gasprice|extcodesize|extcodecopy|returndatasize|returndatacopy|extcodehash" )]
    #[regex(r"blockhash|coinbase|timestamp|number|difficulty|gaslimit|chainid|selfbalance")]
    #[regex(r"pop|mload|mstore8|mstore|sload|sstore|jumpdest|jumpi|jump|pc|msize|gas")]
    #[regex("push([1-9]|[1-2][0-9]|3[0-2])")] // PUSH1-PUSH32
    #[regex("swap([1-9]|1[0-6])")] // SWAP1-SWAP16
    #[regex("dup([1-9]|1[0-6])")] // DUP1-DUP16
    #[regex("log[0-4]")] // LOG0-LOG4
    #[regex(
        r"create2|create|callcode|call|return|delegatecall|staticcall|revert|invalid|selfdestruct"
    )]
    #[regex(r"sha3")]
    Opcode(&'a str),
    /// Jump Label
    #[regex(r"<[a-zA-Z0-9_\\-]+>")]
    JumpLabel(&'a str),
    /// Jump Label
    #[regex(r"[a-zA-Z0-9_\\-]+:")]
    JumpDest(&'a str),
    /// A Comment
    #[regex(r"//.*")] // single line comment
    #[regex("/\\*[^*]*\\*+(?:[^/*][^*]*\\*+)*/")] // multi line comment
    Comment(&'a str),
    /// Identifier
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident(&'a str),
    /// Error
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)] // Whitespace
    Error,
}

impl<'a> fmt::Display for TokenKind<'a> {
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
            TokenKind::JumpLabel(label) => return write!(f, "{}", label),
            TokenKind::JumpDest(label) => return write!(f, "{}", label),
            _ => "<error>",
        };

        write!(f, "{}", x)
    }
}
