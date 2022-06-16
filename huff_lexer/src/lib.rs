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
//! let tok = lexer.next().unwrap();
//! assert_eq!(tok, Token::new(TokenKind::Define, Span::new(0..7)));
//! assert_eq!(lexer.span, Span::new(0..7));
//!
//!
//! // Then we should parse the macro keyword
//! let tok = lexer.next().unwrap();
//! assert_eq!(tok, Token::new(TokenKind::Macro, Span::new(8..13)));
//! assert_eq!(lexer.span, Span::new(8..13));
//!
//!
//! // Then we should get the function name
//! let tok = lexer.next().unwrap();
//! assert_eq!(tok, Token::new(TokenKind::Ident("HELLO_WORLD".to_string()), Span::new(14..25)));
//! assert_eq!(lexer.span, Span::new(14..25));
//!
//! // Then we should have an open paren
//! let tok = lexer.next().unwrap();
//! assert_eq!(tok, Token::new(TokenKind::OpenParen, Span::new(25..26)));
//! assert_eq!(lexer.span, Span::new(25..26));
//!
//! // Lastly, we should have a closing parenthesis
//! let tok = lexer.next().unwrap();
//! assert_eq!(tok, Token::new(TokenKind::CloseParen, Span::new(26..27)));
//! assert_eq!(lexer.span, Span::new(26..27));
//!
//! // We covered the whole source
//! assert_eq!(lexer.span.end, source.len());
//! assert!(lexer.next().is_none());
//! assert!(lexer.eof);
//! ```

#![deny(missing_docs)]
#![allow(dead_code)]
use huff_utils::{span::*, token::*};
use logos::Logos;

/// Lexer
#[derive(Clone)]
pub struct Lexer<'a> {
    /// Source code
    pub source: &'a str,
    /// Current span
    pub span: Span,
    /// End of file
    pub eof: bool,
    inner: logos::Lexer<'a, TokenKind<'a>>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer
    pub fn new(source: &'a str) -> Self {
        Self { source, span: Span::default(), eof: false, inner: TokenKind::lexer(source) }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind_opt = self.inner.next();
        let span = self.inner.span();
        self.span = Span { start: span.start, end: span.end };

        match kind_opt {
            Some(TokenKind::Opcode(op)) => {
                if self.inner.extras.in_scope {
                    // Lex as opcodes token if inside a scope
                    Some(Token { kind: TokenKind::Opcode(op), span: self.span })
                } else {
                    // Lex as identifier otherwise
                    Some(Token { kind: TokenKind::Ident(op), span: self.span })
                }
            }
            Some(kind) => Some(Token::new(kind, self.span)),
            None => {
                self.eof = true;
                None
            }
        }
    }
}
