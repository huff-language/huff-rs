use huff_utils::token::{ Token, TokenKind };
use huff_utils::token::{ Contract, MacroDefinition, Statement, MacroInvocation };

use std::mem::discriminant;

///
#[derive(Debug)]
pub enum ParserError {
    SyntaxError,
}

///
pub struct Parser<'a> {
    ///
    tokens: Vec<Token<'a>>,
    ///
    cursor: usize,
    ///
    current_token: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
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

    fn peek(&self) -> Token<'a> {
        self.tokens.get(self.cursor + 1).unwrap().clone()
    }

    fn peek_behind(&self) -> Token<'a> {
        self.tokens.get(self.cursor - 1).unwrap().clone()
    }

    // pub struct MacroDefinition<'a> {
    //     name: String,
    //     arguments: Vec<String>,
    //     statements: Vec<Statement<'a>>,
    //     takes: usize,
    //     returns: usize,
    // }

    fn parse_macro_args(&mut self) -> Result<Vec<String>, ParserError> {
        let mut args: Vec<String> = Vec::new();

        self.match_kind(TokenKind::OpenParen)?;

        while !self.check(TokenKind::CloseParen) {
            if self.check(TokenKind::Ident("x")) {
                self.match_kind(TokenKind::Ident("x"))?;
                let tok = self.peek_behind().kind;

                match tok {
                    TokenKind::Ident(name) => args.push(name.to_string()),
                    _ => return Err(ParserError::SyntaxError),
                }
            }

            if self.check(TokenKind::Comma) {
                self.consume();
            }
        }

        Ok(args)
    }

    // pub struct MacroInvocation<'a> {
    //     macro_name: String,
    //     args: Vec<&'a Literal>,
    // }

    // fn parse_macro_call(&self) -> Result<MacroInvocation, ParserError> {
    //     let invocation: MacroInvocation;

    //     self.match_kind(TokenKind::Ident("MACRO_NAME"))?;
    //     let tok = self.peek_behind().kind;

    //     match tok {
    //         TokenKind::Ident(name) => invocation.macro_name = name,
    //         _ => return Err(ParserError::SyntaxError),
    //     }

    //     self.parse_macro_call_args()?;
        
    //     Ok()
    // }

    // fn parse_macro_call_args(&self) -> Result<Vec<Literal<'a>>, ParserError> {
    //     self.match_kind(TokenKind::OpenParen)?;
    //     while !self.check(TokenKind::CloseParen) {
    //         match self.current_token.kind {
    //             TokenKind::Literal(_) | TokenKind::Ident(_) => self.consume(),
    //             _ => return Err(ParserError::SyntaxError)
    //         }
    //         if self.check(TokenKind::Comma) {
    //             self.consume();
    //         }
    //     }
    //     self.consume();
    //     Ok(())
    // }

    // fn parse_constant_push(&self) {

    // }

    fn parse_macro_body(&mut self) -> Result<Vec<Statement<'static>>, ParserError> {
        let mut statements: Vec<Statement<'static>> = Vec::new();

        self.match_kind(TokenKind::OpenBrace)?;
        while !self.check(TokenKind::CloseBrace) {
            match self.current_token.kind {
                TokenKind::Literal(val) => { 
                    self.consume();
                    statements.push(Statement::Literal(val));
                },
                TokenKind::Opcode => { 
                    self.consume();
                    statements.push(Statement::Opcode);
                },
                // TokenKind::Ident("MACRO_NAME") => { 
                //     let literals = self.parse_macro_call();
                //     statements.push();
                // },
                _ => return Err(ParserError::SyntaxError)
            }
        }
        self.consume();
        Ok(statements)
    }

    pub fn parse_macro(&mut self) -> Result<MacroDefinition<'a>, ParserError> {
        let macro_name: String;
        let macro_arguments: Vec<String>;
        let macro_takes: usize;
        let macro_returns: usize;
        let macro_statements: Vec<Statement<'static>>;

        self.match_kind(TokenKind::Define)?;
        self.match_kind(TokenKind::Macro)?;
        self.match_kind(TokenKind::Ident("MACRO_NAME"))?;

        let tok = self.peek_behind().kind;
        let macro_ident;

        match tok {
            TokenKind::Ident(name) => macro_ident = name,
            _ => return Err(ParserError::SyntaxError),
        }

        macro_name = macro_ident.to_string();

        macro_arguments = self.parse_macro_args()?;
        self.match_kind(TokenKind::Equal)?;
        self.match_kind(TokenKind::Takes)?;
        self.match_kind(TokenKind::OpenParen)?;
        self.match_kind(TokenKind::Num(1))?;

        let tok = self.peek_behind().kind;
        let takes: usize;

        match tok {
            TokenKind::Num(value) => takes = value,
            _ => return Err(ParserError::SyntaxError),
        }

        macro_takes = takes;

        self.match_kind(TokenKind::CloseParen)?;
        self.match_kind(TokenKind::Returns)?;
        self.match_kind(TokenKind::OpenParen)?;
        self.match_kind(TokenKind::Num(1))?;

        let tok = self.peek_behind().kind;
        let returns: usize;

        match tok {
            TokenKind::Num(value) => returns = value,
            _ => return Err(ParserError::SyntaxError),
        }

        macro_returns = returns;
        macro_statements = self.parse_macro_body()?;

        Ok(MacroDefinition::new(macro_name, macro_arguments, macro_statements, macro_takes, macro_returns))
    }
}