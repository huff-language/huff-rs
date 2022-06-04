//! Huff Parser
//!
//! A parser for the Huff Language.
//!
//! The Huff Parser accepts a vector of Tokens during instantiation.
//!
//! Once instantiated, the parser will construct an AST from the Token Vector when the `parse`
//! method is called.
//!
//! It also exposes a number of practical methods for accessing information about the source code
//! throughout lexing.
//! #### Usage
//!
//! The following example steps through the lexing of a simple, single-line source code macro
//! definition.
//!
//! ```rust
//! use huff_utils::{token::*, span::*};
//! use huff_lexer::{Lexer};
//! use huff_parser::{Parser};
//! ```

#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

use huff_utils::{
    ast::*,
    token::{Token, TokenKind},
};

/// A Parser Error
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum ParserError {
    /// A general syntax error that accepts a message
    SyntaxError(&'static str),
    /// Unexpected type
    UnexpectedType,
    /// Invalid definition
    InvalidDefinition,
    /// Invalid constant value
    InvalidConstantValue,
    /// Invalid macro name
    InvalidMacroName,
    /// Invalid arguments
    InvalidArgs,
    /// Invalid macro call arguments
    InvalidMacroArgs,
    /// Invalid return arguments
    InvalidReturnArgs,
}

/// The Parser
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    // Vector of the tokens
    pub tokens: Vec<Token<'a>>,
    // Current position
    pub cursor: usize,
    /// Current token
    pub current_token: Token<'a>,
}

impl<'a> Parser<'a> {
    /// Public associated function that instantiates a Parser.
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        let initial_token = tokens.get(0).unwrap().clone();
        Self { tokens, cursor: 0, current_token: initial_token }
    }

    /// Parse
    pub fn parse(&mut self) -> Result<(), ParserError> {
        // remove all whitespaces and newlines first
        // NOTE: lexer considers newlines as whitespaces
        self.tokens.retain(|&token| match token.kind {
            TokenKind::Whitespace => false,
            _ => true,
        });
        while !self.check(TokenKind::Eof) {
            self.parse_statement()?;
        }
        Ok(())
    }

    /// Match current token to a type.
    pub fn match_kind(&mut self, kind: TokenKind) -> Result<(), ParserError> {
        if std::mem::discriminant(&mut self.current_token.kind) == std::mem::discriminant(&kind) {
            self.consume();
            Ok(())
        } else {
            println!(
                "Expected current token of kind {} to match {}",
                self.current_token.kind, kind
            );
            Err(ParserError::UnexpectedType)
        }
    }

    /// Check the current token's type against the given type.
    pub fn check(&mut self, kind: TokenKind) -> bool {
        std::mem::discriminant(&mut self.current_token.kind) == std::mem::discriminant(&kind)
    }

    /// Consumes the next token.
    pub fn consume(&mut self) {
        self.current_token = self.peek().unwrap();
        self.cursor += 1;
    }

    /// Take a look at next token without consuming.
    pub fn peek(&mut self) -> Option<Token<'a>> {
        if self.cursor >= self.tokens.len() {
            None
        } else {
            Some(self.tokens.get(self.cursor + 1).unwrap().clone())
        }
    }

    /// Take a look at the previous token.
    pub fn peek_behind(&self) -> Option<Token<'a>> {
        if self.cursor == 0 || self.cursor > self.tokens.len() {
            None
        } else {
            Some(self.tokens.get(self.cursor - 1).unwrap().clone())
        }
    }

    /// Parse a statement.
    fn parse_statement(&mut self) -> Result<(), ParserError> {
        // first token should be keyword "#define"
        self.match_kind(TokenKind::Define)?;
        // match to fucntion, constant or macro
        match self.current_token.kind {
            TokenKind::Function => self.parse_function(),
            TokenKind::Constant => self.parse_constant(),
            TokenKind::Macro => {
                let macro_definitions = self.parse_macro().unwrap();
                Ok(())
            }
            _ => {
                println!(
                    "Invalid definition. Must be a function, event, constant, or macro. Got: {}",
                    self.current_token.kind
                );
                Err(ParserError::InvalidDefinition)
            }
        };
        Ok(())
    }

    /*
        Parse a function.
    */
    fn parse_function(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::Function)?;
        // function name should be next
        self.match_kind(TokenKind::Ident("x"))?;
        self.parse_args(false)?;
        // TODO: Replace with a TokenKind specific to view, payable or nonpayable keywords
        self.match_kind(TokenKind::Ident("FUNC_TYPE"))?;
        self.match_kind(TokenKind::Returns)?;
        self.parse_args(false)?;

        Ok(())
    }

    /*
        Parse a constant.
    */
    fn parse_constant(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::Constant)?;
        self.match_kind(TokenKind::Ident("x"))?;
        self.match_kind(TokenKind::Assign)?;
        match self.current_token.kind {
            TokenKind::FreeStoragePointer | TokenKind::Literal(_) => {
                self.consume();
                Ok(())
            }
            _ => {
                println!(
                    "Constant value must be of kind FreeStoragePointer or Literal. Got: {}",
                    self.current_token.kind
                );
                Err(ParserError::InvalidConstantValue)
            }
        }
    }

    /// Parses a macro.
    ///
    /// It should parse the following : macro MACRO_NAME(args...) = takes (x) returns (n) {...}
    pub fn parse_macro(&mut self) -> Result<MacroDefinition<'a>, ParserError> {
        let macro_name: String;
        let macro_arguments: Vec<String>;
        let macro_takes: usize;
        let macro_returns: usize;
        let macro_statements: Vec<Statement<'static>>;

        self.match_kind(TokenKind::Define)?;
        self.match_kind(TokenKind::Macro)?;
        self.match_kind(TokenKind::Ident("MACRO_NAME"))?;

        let tok = self.peek_behind().unwrap().kind;
        let macro_ident;

        match tok {
            TokenKind::Ident(name) => macro_ident = name,
            _ => {
                println!("Invalid macro name. Must be of kind Ident. Got: {}", tok);
                return Err(ParserError::InvalidMacroName)
            }
        }

        macro_name = macro_ident.to_string();

        macro_arguments = self.parse_args(false)?;
        self.match_kind(TokenKind::Assign)?;
        self.match_kind(TokenKind::Takes)?;
        self.match_kind(TokenKind::OpenParen)?;
        self.match_kind(TokenKind::Num(1))?;

        let tok = self.peek_behind().unwrap().kind;
        let takes: usize = match tok {
            TokenKind::Num(value) => value,
            _ => {
                println!("Invalid macro arguments. Must be of kind Num. Got: {}", tok);
                return Err(ParserError::InvalidMacroArgs)
            }
        };

        macro_takes = takes;

        self.match_kind(TokenKind::CloseParen)?;
        self.match_kind(TokenKind::Returns)?;
        self.match_kind(TokenKind::OpenParen)?;
        self.match_kind(TokenKind::Num(1))?;

        let tok = self.peek_behind().unwrap().kind;
        let returns: usize = match tok {
            TokenKind::Num(value) => value,
            _ => {
                println!("Invalid macro return arguments. Must be of kind Num. Got: {}", tok);
                return Err(ParserError::InvalidReturnArgs)
            }
        };

        macro_returns = returns;
        macro_statements = self.parse_body()?;

        Ok(MacroDefinition::new(
            macro_name,
            macro_arguments,
            macro_statements,
            macro_takes,
            macro_returns,
        ))
    }

    /// Parse the body of a macro.
    ///
    /// Only HEX, OPCODES, labels and MACRO calls should be authorized.
    pub fn parse_body(&mut self) -> Result<Vec<Statement<'static>>, ParserError> {
        let mut statements: Vec<Statement<'static>> = Vec::new();
        self.match_kind(TokenKind::OpenBrace)?;
        while !self.check(TokenKind::CloseBrace) {
            match self.current_token.kind {
                TokenKind::Literal(val) => {
                    self.consume();
                    statements.push(Statement::Literal(val));
                }
                TokenKind::Opcode(o) => {
                    self.consume();
                    statements.push(Statement::Opcode(o));
                }
                // TokenKind::Ident("MACRO_NAME") => {
                //     let literals = self.parse_macro_call();
                //     statements.push();
                // },
                TokenKind::Label(_) => {
                    self.consume();
                }
                TokenKind::OpenBracket => {
                    self.parse_constant_push()?;
                }
                _ => return Err(ParserError::SyntaxError(
                    "Invalid token in macro body. Must be of kind Hex, Opcode, Macro, or Label.",
                )),
            };
        }
        // consume close brace
        self.match_kind(TokenKind::CloseBrace)?;
        Ok(statements)
    }

    /// Parse new lines.
    ///
    /// No-return since newlines are non-essential.
    pub fn parse_newline(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::Whitespace)?;
        while self.check(TokenKind::Whitespace) {
            self.consume();
        }
        Ok(())
    }

    /// Parse arguments
    ///
    /// Arguments can be typed or not. Between parenthesis.
    /// Works for both inputs and outputs.
    /// It should parse the following : (uint256 a, bool b, ...)
    pub fn parse_args(&mut self, name_only: bool) -> Result<Vec<String>, ParserError> {
        let mut args: Vec<String> = Vec::new();
        self.match_kind(TokenKind::OpenParen)?;
        while !self.check(TokenKind::CloseParen) {
            // type comes first
            // TODO: match against TokenKind dedicated to EVM Types (uint256, bytes, ...)
            if name_only {
                self.match_kind(TokenKind::Ident("EVMType"))?
            };
            // naming is optional
            if self.check(TokenKind::Ident("x")) {
                self.match_kind(TokenKind::Ident("x"))?;
                let tok = self.peek_behind().unwrap().kind;

                match tok {
                    TokenKind::Ident(name) => args.push(name.to_string()),
                    _ => {
                        println!("Invalid argument name. Must be of kind Ident. Got: {}", tok);
                        return Err(ParserError::InvalidArgs)
                    }
                }
            }
            // multiple args possible
            if self.check(TokenKind::Comma) {
                self.consume();
            }
        }
        // consume close parenthesis
        self.match_kind(TokenKind::CloseParen)?;
        Ok(args)
    }

    // TODO: Below is from the vi/parser branch

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

    /// Parses the following : (x)
    fn parse_single_arg(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::OpenParen)?;
        self.match_kind(TokenKind::Num(0))?;
        self.match_kind(TokenKind::CloseParen)?;
        Ok(())
    }

    /// Parse call to a macro.
    fn parse_macro_call(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::Ident("MACRO_NAME"))?;
        self.parse_macro_call_args()?;
        Ok(())
    }

    /// Parse the arguments of a macro call.
    fn parse_macro_call_args(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::OpenParen)?;
        while !self.check(TokenKind::CloseParen) {
            // We can pass either directly hex values or labels (without the ":")
            match self.current_token.kind {
                TokenKind::Literal(_) | TokenKind::Ident(_) => self.consume(),
                _ => {
                    println!(
                        "Invalid macro call arguments. Must be of kind Ident or Literal. Got: {}",
                        self.current_token.kind
                    );
                    return Err(ParserError::InvalidMacroArgs)
                }
            }
            if self.check(TokenKind::Comma) {
                self.consume();
            }
        }
        // consume close parenthesis
        self.consume();
        Ok(())
    }

    pub fn parse_constant_push(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::OpenBracket)?;
        self.match_kind(TokenKind::Ident("CONSTANT"))?;
        self.match_kind(TokenKind::CloseBracket)?;
        Ok(())
    }

    /// Parses whitespaces and newlines until none are left.
    pub fn parse_nl_or_whitespace(&mut self) -> Result<(), ParserError> {
        while self.check(TokenKind::Whitespace) {
            self.consume();
        }
        Ok(())
    }
}
