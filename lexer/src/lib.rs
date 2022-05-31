//! Lexer
//!
//! A module for lexical analysis of huff code.

#![deny(missing_docs)]

use std::iter::Peekable;
use std::str::Chars;

use utils::{Report, Reporter, Span, Spanned};

/// ## Lexer
///
/// The lexer encapsulated in a struct.
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    source: &'a str,
    span: Span,
    eof: bool,
}

impl<'a> Lexer<'a> {
    /// Public associated function that instantiates a new lexer.
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            source,
            span: Span::default(),
            eof: false,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn peek(&self, n: usize) -> Option<char> {
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

    pub fn current_span(&self) -> Span {
        if self.eof {
            Span::EOF
        } else {
            self.span
        }
    }
}
