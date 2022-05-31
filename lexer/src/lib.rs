//! Lexer
//!
//! A module for lexical analysis of huff code.

#![deny(missing_docs)]

use std::{iter::Peekable, str::Chars};

use utils::{
    error::{LexicalError, LexicalErrorKind},
    span::Span,
    token::{Token, TokenKind},
};

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

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn peekn(&self, n: usize) -> Option<char> {
        self.chars.clone().nth(n)
    }

    fn chomp(&mut self) -> Option<char> {
        self.chars.next().map(|x| {
            self.span.end += 1;
            x
        })
    }

    fn slice(&self) -> &'a str {
        &self.source[self.span.range().unwrap()]
    }

    fn reset(&mut self) {
        self.span.start = self.span.end;
    }

    fn chomp_while(&mut self, f: impl Fn(&char) -> bool + Copy) {
        while self.peek().map(|x| f(&x)).unwrap_or(false) {
            self.chomp();
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
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        use LexicalErrorKind::*;
        use TokenKind::*;

        self.reset();
        if let Some(ch) = self.chomp() {
            let kind = match ch {
                '+' => Add,
                '-' => Sub,
                '/' => Div,
                '*' => Mul,
                '0'..='9' => {
                    self.chomp_while(char::is_ascii_digit);
                    Num(self.slice().parse().unwrap())
                }
                ch if ch.is_ascii_whitespace() => {
                    self.chomp_while(char::is_ascii_whitespace);
                    Whitespace
                }
                '"' => loop {
                    match self.peek() {
                        Some('"') => {
                            self.chomp();
                            let str = self.slice();
                            break TokenKind::Str(&str[1..str.len() - 1])
                        }
                        Some('\\') if matches!(self.peekn(1), Some('\\') | Some('"')) => {
                            self.chomp();
                        }
                        Some(_) => {}
                        None => {
                            self.eof = true;
                            return Some(Err(LexicalError::new(UnexpectedEof, self.span)))
                        }
                    }

                    self.chomp();
                },
                ';' => TokenKind::Semi,
                '=' => TokenKind::Assign,
                '(' => TokenKind::OpenParen,
                ')' => TokenKind::CloseParen,
                ',' => TokenKind::Comma,
                ch if ch.is_alphabetic() => {
                    self.chomp_while(|c| c.is_alphanumeric());
                    TokenKind::Ident(self.slice())
                }
                ch => return Some(Err(LexicalError::new(InvalidCharacter(ch), self.span))),
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
