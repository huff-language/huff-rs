//! ## Huff Lexer
//!
//! Lexical analyzer for the huff language.
//!
//! #### Usage
//!
//! ```rust
//! use huff_lexer::{Lexer};
//!
//! // Instantiate a new lexer
//! let source = "#define macro HELLO_WORLD()";
//! let lexer = Lexer::new(source);
//! assert_eq!(lexer.source, source);
//! ```

#![deny(missing_docs)]
#![allow(dead_code)]

use huff_utils::{error::*, span::*, token::*};
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
}

impl<'a> Lexer<'a> {
    /// Public associated function that instantiates a new lexer.
    pub fn new(source: &'a str) -> Self {
        Self { chars: source.chars().peekable(), source, span: Span::default(), eof: false }
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
                    if let Some(ch2) = self.consume() {
                        match ch2 {
                            '/' => {
                                // Consume until newline
                                self.dyn_consume(|c| *c != '\n');
                                TokenKind::Comment(self.slice())
                            }
                            '*' => {
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

                    // Match on the include keyword
                    let include_keyword = "#include";
                    let peeked = self.peeknchars(include_keyword.len() - 1);
                    if include_keyword == peeked {
                        self.dyn_consume(|c| c.is_alphabetic());
                        found_kind = Some(TokenKind::Include);
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
                    // Check for macro keyword
                    let macro_keyword = "macro";
                    let peeked = self.peeknchars(macro_keyword.len() - 1);
                    if macro_keyword == peeked {
                        self.dyn_consume(|c| c.is_alphabetic());
                        TokenKind::Macro
                    } else {
                        self.dyn_consume(|c| c.is_alphanumeric() || c.eq(&'_'));
                        TokenKind::Ident(self.slice())
                    }
                }

                '+' => TokenKind::Add,
                '-' => TokenKind::Sub,
                '*' => TokenKind::Mul,
                '0'..='9' => {
                    self.dyn_consume(char::is_ascii_digit);
                    TokenKind::Num(self.slice().parse().unwrap())
                }
                ch if ch.is_ascii_whitespace() => {
                    self.dyn_consume(char::is_ascii_whitespace);
                    TokenKind::Whitespace
                }
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
                ';' => TokenKind::Semi,
                '=' => TokenKind::Assign,
                '(' => TokenKind::OpenParen,
                ')' => TokenKind::CloseParen,
                ',' => TokenKind::Comma,

                ch => {
                    return Some(Err(LexicalError::new(
                        LexicalErrorKind::InvalidCharacter(ch),
                        self.span,
                    )))
                }
            };

            if self.peek().is_none() {
                self.eof = true;
            }

            let token = Token { kind, span: self.span };

            return Some(Ok(token))
        }

        self.eof = true;
        None
    }
}
