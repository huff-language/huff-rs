//! ## Huff Lexer
//!
//! Lexical analyzer for the huff language.
//!
//! The Huff Lexer is instantiable with a string representing the source code.
//!
//! Once instantiated, the lexer can be used to iterate over the tokens in the source code.
//! It also exposes a number of practical methods for accessing information about the source code
//! throughout lexing.
//!
//! #### Usage
//!
//! The following example steps through the lexing of a simple, single-line source code macro
//! definition.
//!
//! ```rust
//! use huff_utils::{token::*, span::*};
//! use huff_lexer::{Lexer};
//!
//! // Instantiate a new lexer
//! let source = "#define macro HELLO_WORLD()";
//! let mut lexer = Lexer::new(source);
//! assert_eq!(lexer.source, source);
//!
//! // This token should be a Define identifier
//! let tok = lexer.next().unwrap().unwrap();
//! assert_eq!(tok, Token::new(TokenKind::Define, Span::new(0..7)));
//! assert_eq!(lexer.span, Span::new(0..7));
//!
//! // The next token should be the whitespace
//! let tok = lexer.next().unwrap().unwrap();
//! assert_eq!(tok, Token::new(TokenKind::Whitespace, Span::new(7..8)));
//! assert_eq!(lexer.span, Span::new(7..8));
//!
//! // Then we should parse the macro keyword
//! let tok = lexer.next().unwrap().unwrap();
//! assert_eq!(tok, Token::new(TokenKind::Macro, Span::new(8..13)));
//! assert_eq!(lexer.span, Span::new(8..13));
//!
//! // The next token should be another whitespace
//! let tok = lexer.next().unwrap().unwrap();
//! assert_eq!(tok, Token::new(TokenKind::Whitespace, Span::new(13..14)));
//! assert_eq!(lexer.span, Span::new(13..14));
//!
//! // Then we should get the function name
//! let tok = lexer.next().unwrap().unwrap();
//! assert_eq!(tok, Token::new(TokenKind::Ident("HELLO_WORLD"), Span::new(14..25)));
//! assert_eq!(lexer.span, Span::new(14..25));
//!
//! // Then we should have an open paren
//! let tok = lexer.next().unwrap().unwrap();
//! assert_eq!(tok, Token::new(TokenKind::OpenParen, Span::new(25..26)));
//! assert_eq!(lexer.span, Span::new(25..26));
//!
//! // Lastly, we should have a closing parenthesis
//! let tok = lexer.next().unwrap().unwrap();
//! assert_eq!(tok, Token::new(TokenKind::CloseParen, Span::new(26..27)));
//! assert_eq!(lexer.span, Span::new(26..27));
//!
//! // We covered the whole source
//! lexer.next();
//! assert_eq!(lexer.span.end, source.len());
//! assert!(lexer.eof);
//! assert!(lexer.next().is_none());
//! ```

#![deny(missing_docs)]
#![allow(dead_code)]

use huff_utils::{error::*, span::*, token::*, evm::*};
use std::{iter::Peekable, str::Chars};

/// ## Lexer
///
/// The lexer encapsulated in a struct.
pub struct Lexer<'a> {
    /// The source code as peekable chars.
    pub chars: Peekable<Chars<'a>>,
    /// The raw source code.
    pub source: &'a str,
    /// The current lexing span.
    pub span: Span,
    /// If the lexer has reached the end of file.
    pub eof: bool,
    /// EOF Token has been returned.
    pub eof_returned: bool,
}

impl<'a> Lexer<'a> {
    /// Public associated function that instantiates a new lexer.
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            source,
            span: Span::default(),
            eof: false,
            eof_returned: false,
        }
    }

    /// Public associated function that returns the current lexing span.
    pub fn current_span(&self) -> Span {
        if self.eof {
            Span::EOF
        } else {
            self.span
        }
    }

    /// Try to peek at the next character from the source
    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// Try to peek at the nth character from the source
    pub fn nthpeek(&mut self, n: usize) -> Option<char> {
        self.chars.clone().nth(n)
    }

    /// Try to peek at next n characters from the source
    pub fn peeknchars(&mut self, n: usize) -> String {
        let mut newspan: Span = self.span;
        newspan.end += n;
        // Break with an empty string if the bounds are exceeded
        if newspan.end > self.source.len() {
            return String::default()
        }
        self.source[newspan.range().unwrap()].to_string()
    }

    /// Peek n chars from a given start point in the source
    pub fn peekncharsfrom(&mut self, n: usize, from: usize) -> String {
        self.source[Span::new(from..(from + n)).range().unwrap()].to_string()
    }

    /// Gets the current slice of the source code covered by span
    pub fn slice(&self) -> &'a str {
        &self.source[self.span.range().unwrap()]
    }

    /// Consumes the characters
    pub fn consume(&mut self) -> Option<char> {
        self.chars.next().map(|x| {
            self.span.end += 1;
            x
        })
    }

    /// Consumes n characters
    pub fn nconsume(&mut self, count: usize) {
        for _ in 0..count {
            let _ = self.consume();
        }
    }

    /// Consume characters until a sequence matches
    pub fn seq_consume(&mut self, word: &str) {
        let mut current_pos = self.span.start;
        while self.peek() != None {
            let peeked = self.peekncharsfrom(word.len(), current_pos);
            if word == peeked {
                break
            }
            self.consume();
            current_pos += 1;
        }
    }

    /// Dynamically consumes characters based on filters
    pub fn dyn_consume(&mut self, f: impl Fn(&char) -> bool + Copy) {
        while self.peek().map(|x| f(&x)).unwrap_or(false) {
            self.consume();
        }
    }

    /// Resets the Lexer's span
    pub fn reset(&mut self) {
        self.span.start = self.span.end;
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexicalError>;

    /// Iterates over the source code
    fn next(&mut self) -> Option<Self::Item> {
        self.reset();
        if let Some(ch) = self.consume() {
            let kind = match ch {
                // Comments
                '/' => {
                    if let Some(ch2) = self.peek() {
                        match ch2 {
                            '/' => {
                                self.consume();
                                // Consume until newline
                                self.dyn_consume(|c| *c != '\n');
                                TokenKind::Comment(self.slice())
                            }
                            '*' => {
                                self.consume();
                                // Consume until next '*/' occurance
                                self.seq_consume("*/");
                                TokenKind::Comment(self.slice())
                            }
                            _ => TokenKind::Div,
                        }
                    } else {
                        TokenKind::Div
                    }
                }
                // # keywords
                '#' => {
                    let mut found_kind: Option<TokenKind> = None;

                    // Match exactly on define keyword
                    let define_keyword = "#define";
                    let peeked = self.peeknchars(define_keyword.len() - 1);
                    if define_keyword == peeked {
                        self.dyn_consume(|c| c.is_alphabetic());
                        found_kind = Some(TokenKind::Define);
                    }

                    if found_kind == None {
                        // Match on the include keyword
                        let include_keyword = "#include";
                        let peeked = self.peeknchars(include_keyword.len() - 1);
                        if include_keyword == peeked {
                            self.dyn_consume(|c| c.is_alphabetic());
                            found_kind = Some(TokenKind::Include);
                        }
                    }

                    if let Some(kind) = found_kind {
                        kind
                    } else {
                        // Otherwise we don't support # prefixed indentifiers
                        return Some(Err(LexicalError::new(
                            LexicalErrorKind::InvalidCharacter('#'),
                            self.current_span(),
                        )))
                    }
                }
                // Alphabetical characters
                ch if ch.is_alphabetic() => {
                    let mut found_kind: Option<TokenKind> = None;

                    // Check for macro keyword
                    let macro_keyword = "macro";
                    let peeked = self.peeknchars(macro_keyword.len() - 1);
                    if macro_keyword == peeked {
                        self.dyn_consume(|c| c.is_alphabetic());
                        found_kind = Some(TokenKind::Macro);
                    }

                    // Check for the function keyword
                    if found_kind == None {
                        let function_keyword = "function";
                        let peeked = self.peeknchars(function_keyword.len() - 1);
                        if function_keyword == peeked {
                            self.dyn_consume(|c| c.is_alphabetic());
                            found_kind = Some(TokenKind::Function);
                        }
                    }

                    // Check for the constant keyword
                    if found_kind == None {
                        let constant_keyword = "constant";
                        let peeked = self.peeknchars(constant_keyword.len() - 1);
                        if constant_keyword == peeked {
                            self.dyn_consume(|c| c.is_alphabetic());
                            found_kind = Some(TokenKind::Constant);
                        }
                    }

                    // Check for the takes keyword
                    if found_kind == None {
                        let takes_key = "takes";
                        let peeked = self.peeknchars(takes_key.len() - 1);
                        if takes_key == peeked {
                            self.dyn_consume(|c| c.is_alphabetic());
                            found_kind = Some(TokenKind::Takes);
                        }
                    }

                    // Check for the returns keyword
                    if found_kind == None {
                        let returns_key = "returns";
                        let peeked = self.peeknchars(returns_key.len() - 1);
                        if returns_key == peeked {
                            self.dyn_consume(|c| c.is_alphabetic());
                            found_kind = Some(TokenKind::Returns);
                        }
                    }

                    // Check for macro keyword
                    let fsp = "FREE_STORAGE_POINTER";
                    let peeked = self.peeknchars(fsp.len() - 1);
                    if fsp == peeked {
                        self.dyn_consume(|c| c.is_alphabetic() || c.eq(&'_'));
                        // Consume the parenthesis following the FREE_STORAGE_POINTER
                        if let Some('(') = self.peek() {
                            self.consume();
                        }
                        if let Some(')') = self.peek() {
                            self.consume();
                        }
                        found_kind = Some(TokenKind::FreeStoragePointer);
                    }

                    // goes over all opcodes
                    for opcode in OPCODES {
                        let peeked = self.peeknchars(opcode.len() - 1);
                        if opcode == peeked {
                            self.dyn_consume(|c| c.is_alphanumeric());
                            found_kind = Some(TokenKind::Opcode(
                                OPCODES_MAP.get(opcode).unwrap().to_owned(),
                            ));
                            break;
                        }
                    }

                    if let Some(kind) = found_kind {
                        kind
                    } else {
                        self.dyn_consume(|c| c.is_alphanumeric() || c.eq(&'_'));
                        TokenKind::Ident(self.slice())
                    }
                }
                // If it's the start of a hex literal
                ch if ch == '0' && self.peek().unwrap() == 'x' => {
                    self.dyn_consume(|c| c.is_numeric() || c.eq(&'x'));
                    TokenKind::Literal(self.slice())
                }
                '=' => TokenKind::Assign,
                '(' => TokenKind::OpenParen,
                ')' => TokenKind::CloseParen,
                '[' => TokenKind::OpenBracket,
                ']' => TokenKind::CloseBracket,
                '{' => TokenKind::OpenBrace,
                '}' => TokenKind::CloseBrace,
                '+' => TokenKind::Add,
                '-' => TokenKind::Sub,
                '*' => TokenKind::Mul,
                // NOTE: TokenKind::Div is lexed further up since it overlaps with comment
                // identifiers
                ',' => TokenKind::Comma,
                '0'..='9' => {
                    self.dyn_consume(char::is_ascii_digit);
                    TokenKind::Num(self.slice().parse().unwrap())
                }
                // Lexes Spaces and Newlines as Whitespace
                ch if ch.is_ascii_whitespace() => {
                    self.dyn_consume(char::is_ascii_whitespace);
                    TokenKind::Whitespace
                }
                // String literals
                '"' => loop {
                    match self.peek() {
                        Some('"') => {
                            self.consume();
                            let str = self.slice();
                            break TokenKind::Str(&str[1..str.len() - 1])
                        }
                        Some('\\') if matches!(self.nthpeek(1), Some('\\') | Some('"')) => {
                            self.consume();
                        }
                        Some(_) => {}
                        None => {
                            self.eof = true;
                            return Some(Err(LexicalError::new(
                                LexicalErrorKind::UnexpectedEof,
                                self.span,
                            )))
                        }
                    }
                    self.consume();
                },
                // Allow string literals to be wrapped by single quotes
                '\'' => loop {
                    match self.peek() {
                        Some('\'') => {
                            self.consume();
                            let str = self.slice();
                            break TokenKind::Str(&str[1..str.len() - 1])
                        }
                        Some('\\') if matches!(self.nthpeek(1), Some('\\') | Some('\'')) => {
                            self.consume();
                        }
                        Some(_) => {}
                        None => {
                            self.eof = true;
                            return Some(Err(LexicalError::new(
                                LexicalErrorKind::UnexpectedEof,
                                self.span,
                            )))
                        }
                    }
                    self.consume();
                },
                // At this point, the source code has an invalid or unsupported token
                ch => {
                    return Some(Err(LexicalError::new(
                        LexicalErrorKind::InvalidCharacter(ch),
                        self.span,
                    )))
                }
            };

            let token = Token { kind, span: self.span };

            return Some(Ok(token))
        }

        // Mark EOF
        self.eof = true;

        // If we haven't returned an eof token, return one
        if !self.eof_returned {
            self.eof_returned = true;
            return Some(Ok(Token { kind: TokenKind::Eof, span: self.span }))
        }

        None
    }
}
