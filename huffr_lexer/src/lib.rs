use std::{iter::Peekable, str::Chars};

enum OpcodeType {
    // Opcode list goes here.
}

enum TokenType {
    Opcode(OpcodeType),
    Whitespace,
    // Expand with more types.
}

struct Span {
    start: usize,
    end: usize,
}

struct Token {
    ttype: TokenType,
    span: Span,
}

struct Lexer<'a> {
    pos: usize,
    chars: Peekable<Chars<'a>>,
    eof: bool,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self { chars: source.chars().peekable(), pos: 0, eof: false }
    }

    fn consume() -> Option<char> {
        // Move the lexer along one character.
        Some('g')
    }

    fn peek_n(&self, n: usize) -> Option<char> {
        self.chars.clone().nth(n)
    }
}

impl<'a> Iterator for Lexer<'a> {
    // We need to think about errors.
    type Item = Result<Token, ()>;

    // The bulk of the lexing logic can reside here.
    fn next(&mut self) -> Option<Self::Item> {
        Some(Ok(Token { ttype: TokenType::Whitespace, span: Span { start: 0, end: 0 } }))
    }
}
