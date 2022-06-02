use huff_utils::token::{ Token, TokenKind };
use huff_utils::token::{ Contract, MacroDefinition };

use std::mem::discriminant;

enum ParserError {
    SyntaxError,
}

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    cursor: usize,
    current_token: Token<'a>,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token<'a>>) -> Self {
        let initial_token = tokens.get(0).unwrap().clone();
        Self {
            tokens: tokens,
            cursor: 0,
            current_token: initial_token
        }
    }

    fn match_kind(&mut self, kind: TokenKind) -> Result<(), ParserError> {
        if std::mem::discriminant(&mut self.current_token.kind) == std::mem::discriminant(&kind) {
            self.consume();
            Ok(())
        } else {
            Err(ParserError::SyntaxError)
        }
    }

    fn check(&mut self, kind: TokenKind) -> bool {
        std::mem::discriminant(&mut self.current_token.kind) == std::mem::discriminant(&kind)
    }

    fn consume(&mut self) {
        self.current_token = self.peek();
        self.cursor += 1;
    }

    fn peek(&mut self) -> Token<'a> {
        self.tokens.get(self.cursor + 1).unwrap().clone()
    }

    // pub struct MacroDefinition<'a> {
    //     name: String,
    //     arguments: Vec<String>,
    //     statements: Vec<Statement<'a>>,
    //     takes: usize,
    //     returns: usize,
    // }

    fn parse_macro(&self) -> Result<MacroDefinition<'a>, ParserError> {
        let macro_def: MacroDefinition;

        self.match_kind(TokenKind::Define)?;
        self.match_kind(TokenKind::Macro)?;
        self.match_kind(TokenKind::Ident("MACRO_NAME"))?;

        Ok(macro_def)
    }
}