//! ## Huff Lexer
//!
//! Lexical analyzer for the huff language.
//!
//! #### Usage
//!
//! ```rust
//! use huff_lexer::{Lexer, Span};
//!
//! // Instantiate a new lexer
//! let source = "#define macro HELLO_WORLD()";
//! let lexer = Lexer::new(source);
//! assert_eq!(lexer.source, source);
//!
//! ```

#![deny(missing_docs)]
#![allow(dead_code)]

use huff_utils::{error::*, span::*, token::*};
use std::{iter::Peekable, str::Chars};

#[cfg(test)]
mod tests;

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
    fn nthpeek(&mut self, n: usize) -> Option<char> {
        self.chars.clone().nth(n)
    }

    /// Try to peek at next n characters from the source
    fn peeknchars(&mut self, n: usize) -> String {
        let mut newspan: Span = self.span;
        newspan.end += n;
        self.source[newspan.range().unwrap()].to_string()
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

    /// Consume characters until a sequence matches
    pub fn seq_consume(&mut self, seq: &mut Peekable<Chars<'a>>) {
        while self.peek() != None {
            if self.peek().unwrap() == *seq.peek().unwrap() {
                seq.next();
            }
            self.consume();
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
                // // Comments
                // '/' => {
                //     if let Some(ch2) = self.consume() {
                //         match ch2 {
                //             '/' => {
                //                 self.seq_consume(&mut "\n".chars().peekable());
                //                 TokenKind::Comment(self.slice().to_string())
                //             }
                //             '*' => {
                //                 self.seq_consume(&mut "*/".chars().peekable());
                //                 TokenKind::Comment(self.slice().to_string())
                //             }
                //             _ => TokenKind::Div,
                //         }
                //     } else {
                //         TokenKind::Div
                //     }
                // }
                // // Definitions
                // '#' => {
                //     if let Some(ch2) = self.consume() {
                //         match ch2 {
                //             '#' => {
                //                 self.seq_consume(&mut "\n".chars().peekable());
                //
                // TokenKind::Definition(Definition::Macro(self.slice().to_string()))
                //             }
                //             '@' => {
                //                 self.seq_consume(&mut "\n".chars().peekable());
                //
                // TokenKind::Definition(Definition::Import(self.slice().to_string()))
                //             }
                //             _ => TokenKind::Div,
                //         }
                //     } else {
                //         TokenKind::Div
                //     }
                // }
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
                ch if ch.is_alphabetic() => {
                    self.dyn_consume(|c| c.is_alphanumeric());
                    TokenKind::Ident(self.slice())
                }
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
