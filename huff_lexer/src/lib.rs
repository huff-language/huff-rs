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
//! assert_eq!(lexer.span.end, source.len());
//! assert!(lexer.eof);
//! ```

#![deny(missing_docs)]
#![allow(dead_code)]
use bytes::BytesMut;
use huff_utils::{error::*, evm::*, span::*, token::*};
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
    /// The previous lexing span. (used for lookbacks)
    pub prev_span: Span,
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
            prev_span: Span::default(),
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

    /// Get the length of the previous lexing span.
    pub fn prev_span_len(&self) -> usize {
        self.prev_span.end - self.prev_span.start
    }

    /// Try to peek at the next character from the source
    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// Try to peek at the nth character from the source
    pub fn nth_peek(&mut self, n: usize) -> Option<char> {
        self.chars.clone().nth(n)
    }

    /// Try to peek at next n characters from the source
    pub fn peek_n_chars(&mut self, n: usize) -> String {
        let mut newspan: Span = self.span;
        newspan.end += n;
        // Break with an empty string if the bounds are exceeded
        if newspan.end > self.source.len() {
            return String::default()
        }
        self.source[newspan.range().unwrap()].to_string()
    }

    /// Peek n chars from a given start point in the source
    pub fn peek_n_chars_from(&mut self, n: usize, from: usize) -> String {
        self.source[Span::new(from..(from + n)).range().unwrap()].to_string()
    }

    /// Try to look back `dist` chars from `span.start`, but return an empty string if
    /// `self.span.start - dist` will underflow.
    pub fn try_look_back(&mut self, dist: usize) -> String {
        match self.span.start.checked_sub(dist) {
            Some(n) => self.peek_n_chars_from(dist - 1, n),
            None => String::default(),
        }
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
            let peeked = self.peek_n_chars_from(word.len(), current_pos);
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
        self.prev_span = self.span;
        self.span.start = self.span.end;
    }

    /// Check if a given keyword follows the keyword rules in the `source`. If not, it is a
    /// `TokenKind::Ident`.
    ///
    /// Rules:
    /// - The `macro`, `function` and `constant` keywords must be preceded by a `#define` keyword.
    /// - The `takes` keyword must be preceded by an assignment operator: `=`.
    /// - The `returns` keyword must be succeeded by an open parenthesis and must *not* be succeeded
    ///   by a colon or preceded by the keyword `function`
    pub fn check_keyword_rules(&mut self, found_kind: &Option<TokenKind>) -> bool {
        match found_kind {
            Some(TokenKind::Macro) | Some(TokenKind::Function) | Some(TokenKind::Constant) => {
                let define_key = "#define";
                self.try_look_back(self.prev_span_len() + define_key.len()).trim() == define_key
            }
            Some(TokenKind::Takes) => {
                let assign = "=";
                self.try_look_back(self.prev_span_len() + assign.len()).trim() == assign
            }
            Some(TokenKind::Returns) => {
                let function_key = "function";
                // Allow for loose and tight syntax (e.g. `returns (0)` & `returns(0)`)
                self.peek_n_chars_from(2, self.span.end).trim().starts_with('(') &&
                    self.try_look_back(self.prev_span_len() + function_key.len()).trim() !=
                        function_key &&
                    self.peek_n_chars_from(1, self.span.end) != ":"
            }
            _ => true,
        }
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

                    let keys = [("#define", TokenKind::Define), ("#include", TokenKind::Include)];
                    for (key, kind) in &keys {
                        let peeked = self.peek_n_chars(key.len() - 1);

                        if *key == peeked {
                            self.dyn_consume(|c| c.is_alphabetic());
                            found_kind = Some(*kind);
                            break
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

                    let keys = [
                        ("macro", TokenKind::Macro),
                        ("function", TokenKind::Function),
                        ("constant", TokenKind::Constant),
                        ("takes", TokenKind::Takes),
                        ("returns", TokenKind::Returns),
                    ];
                    for (key, kind) in &keys {
                        let peeked = self.peek_n_chars(key.len() - 1);

                        if *key == peeked {
                            self.dyn_consume(|c| c.is_alphabetic());
                            found_kind = Some(*kind);
                            break
                        }
                    }

                    // Check to see if the found kind is, in fact, a keyword and not the name of
                    // a function. If it is, set `found_kind` to `None` so that it is set to a
                    // `TokenKind::Ident` in the following control flow.
                    if !self.check_keyword_rules(&found_kind) {
                        found_kind = None;
                    }

                    // Check for macro keyword
                    let fsp = "FREE_STORAGE_POINTER";
                    let peeked = self.peek_n_chars(fsp.len() - 1);
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
                        let peeked = self.peek_n_chars(opcode.len() - 1);
                        if opcode == peeked {
                            self.dyn_consume(|c| c.is_alphanumeric());
                            found_kind = Some(TokenKind::Opcode(
                                OPCODES_MAP.get(opcode).unwrap().to_owned(),
                            ));
                            break
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
                    self.dyn_consume(|c| {
                        c.is_numeric() ||
                            // Match a-f, A-F, and 'x'
                            // Note: This still allows for invalid hex, as it doesn't care if
                            // there are multiple 'x' values in the literal.
                            matches!(c, '\u{0041}'..='\u{0046}' | '\u{0061}'..='\u{0066}' | 'x')
                    });
                    let mut arr: [u8; 32] = Default::default();
                    let mut buf = BytesMut::from(self.slice());
                    buf.resize(32, 0);
                    arr.copy_from_slice(buf.as_ref());
                    TokenKind::Literal(arr)
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
                        Some('\\') if matches!(self.nth_peek(1), Some('\\') | Some('"')) => {
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
                        Some('\\') if matches!(self.nth_peek(1), Some('\\') | Some('\'')) => {
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

            if self.peek().is_none() {
                self.eof = true;
            }

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
