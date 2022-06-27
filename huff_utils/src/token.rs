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

/// Lexing context
#[derive(Clone, Eq, PartialEq, Copy)]
pub enum Context {
    /// Global scope
    Global,
    /// Macro body/scope
    Macro,
    /// Function/Event Args
    Args,
}

impl Default for Context {
    fn default() -> Self {
        Context::Global
    }
}

/// Extra data
#[derive(Clone, Default)]
pub struct TokenExtras {
    /// Lexing context
    pub context: Context,
}

// fn to_primitive_type<'a>(lex: &mut Lexer<'a, TokenKind<'a>>) -> Option<PrimitiveEVMType> {
//     let slice = lex.slice();
//     Some(PrimitiveEVMType::try_from(slice.to_string()).unwrap())
// }

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
    /// "event" keyword
    #[token("event")]
    Event,
    /// "constant" keyword
    #[token("constant")]
    Constant,
    /// "takes" keyword
    #[token("takes")]
    Takes,
    /// "returns" keyword
    #[token("returns")]
    Returns,
    /// "view" keyword
    #[token("view")]
    View,
    /// "pure" keyword
    #[token("pure")]
    Pure,
    /// "payable" keyword
    #[token("payable")]
    Payable,
    /// "nonpayable" keyword
    #[token("nonpayable")]
    NonPayable,
    /// "indexed" keyword
    #[token("indexed")]
    Indexed,
    /// "FREE_STORAGE_POINTER()" keyword
    #[token(r"FREE_STORAGE_POINTER()")]
    FreeStoragePointer,
    /// Equal Sign
    #[token("=")]
    Assign,
    /// An open parenthesis
    #[token("(", |lex| {
        if lex.extras.context == Context::Global {
            lex.extras.context = Context::Args;
        }
    })]
    OpenParen,
    /// A close parenthesis
    #[token(")", |lex| {
        if lex.extras.context == Context::Args {
            lex.extras.context = Context::Global;
        }
    })]
    CloseParen,
    /// An open bracket
    #[token("[")]
    OpenBracket,
    /// A close bracket
    #[token("]")]
    CloseBracket,
    /// An open brace
    #[token("{", |lex| lex.extras.context = Context::Macro)]
    OpenBrace,
    /// A close brace
    #[token("}", |lex| lex.extras.context = Context::Global)]
    CloseBrace,
    /// A Less-Than Angle Bracket
    #[token("<")]
    LeftAngle,
    /// A Greater-Than Angle Bracket
    #[token(">")]
    RightAngle,
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
    /// Primitive Types
    // NOTE: "address" type is disambiguated in the lexer
    // string
    #[token("string")]
    // bool
    #[token("bool")]
    // bytes
    #[token("bytes")]
    // bytesN
    #[regex(r"bytes([1-9]|[1-2][0-9]|3[1-2])")]
    // uintN
    #[regex(r"uint(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)")]
    // intN
    #[regex(r"int(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)")]
    PrimitiveType(&'a str),
    /// Array type
    // string[]
    #[regex(r"string(\[[1-9]*\])+")]
    // address[]
    #[regex(r"address(\[[1-9]*\])+")]
    //  bool[]
    #[regex(r"bool(\[[1-9]*\])+")]
    // bytes[]
    #[regex(r"bytes(\[[1-9]*\])+")]
    // bytesN[]
    #[regex(r"bytes([1-9]|[1-2][0-9]|3[1-2])(\[[1-9]*\])+")]
    // uintN[]
    #[regex(r"uint(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)(\[[1-9]*\])+")]
    //  intN[]
    #[regex(r"int(8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)(\[[1-9]*\])+")]
    ArrayType(&'a str),
    /// Opcodes
    // NOTE: "address" opcode is disambiguated in the lexer
    #[regex(r"stop|addmod|add|mulmod|mul|sub|div|sdiv|mod|smod|exp|signextend")]
    #[regex(r"lt|gt|slt|sgt|eq|iszero|and|or|xor|not|byte|shl|shr|sar")]
    #[regex(r"balance|origin|caller|callvalue|calldataload|calldatasize|calldatacopy|codesize|codecopy|gasprice|extcodesize|extcodecopy|returndatasize|returndatacopy|extcodehash")]
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
    #[regex(r"[a-zA-Z0-9_\\-]+:")]
    Label(&'a str),
    /// A Jump table
    #[token("jumptable")]
    JumpTable,
    /// A Packed jump table
    #[token("jumptable__packed")]
    JumpTablePacked,
    /// A code table
    #[token("table")]
    CodeTable,
    /// A built-in function
    #[regex(r"__(codesize|tablesize|tablestart)")]
    BuiltinFunction(&'a str),
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
            TokenKind::Event => "event",
            TokenKind::Constant => "constant",
            TokenKind::View => "view",
            TokenKind::Pure => "pure",
            TokenKind::Payable => "payable",
            TokenKind::NonPayable => "nonpayable",
            TokenKind::Indexed => "indexed",
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
            TokenKind::LeftAngle => "<",
            TokenKind::RightAngle => ">",
            TokenKind::Add => "+",
            TokenKind::Sub => "+",
            TokenKind::Mul => "*",
            TokenKind::Comma => ",",
            TokenKind::Num(num) => return write!(f, "{}", num),
            TokenKind::Str(value) => return write!(f, "{}", value),
            TokenKind::Hex(value) => return write!(f, "{}", value),
            TokenKind::PrimitiveType(value) => return write!(f, "{}", value),
            TokenKind::ArrayType(value) => return write!(f, "{}", value),
            TokenKind::Opcode(o) => return write!(f, "{}", o),
            TokenKind::Label(label) => return write!(f, "{}", label),
            TokenKind::JumpTable => "jumptable",
            TokenKind::JumpTablePacked => "jumptable__packed",
            TokenKind::CodeTable => "table",
            TokenKind::BuiltinFunction(s) => return write!(f, "BuiltinFunction({})", s),
            TokenKind::Error => "<error>",
        };

        write!(f, "{}", x)
    }
}
