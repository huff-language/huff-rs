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
    /// Invalid name (macro, event, function, constant)
    InvalidName,
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
    /// Vector of the tokens
    pub tokens: Vec<Token<'a>>,
    /// Current position
    pub cursor: usize,
    /// Current token
    pub current_token: Token<'a>,
}

impl<'a> Parser<'a> {
    /// Public associated function that instantiates a Parser.
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        let initial_token = *tokens.get(0).unwrap();
        Self { tokens, cursor: 0, current_token: initial_token }
    }

    /// Parse
    pub fn parse(&mut self) -> Result<(), ParserError> {
        // remove all whitespaces and newlines first
        // NOTE: lexer considers newlines as whitespaces
        self.tokens.retain(|&token| !matches!(token.kind, TokenKind::Whitespace));
        while !self.check(TokenKind::Eof) {
            self.parse_definition()?;
        }
        Ok(())
    }

    /// Match current token to a type.
    pub fn match_kind(&mut self, kind: TokenKind) -> Result<TokenKind, ParserError> {
        if std::mem::discriminant(&self.current_token.kind) == std::mem::discriminant(&kind) {
            let curr_kind: TokenKind = self.current_token.kind;
            self.consume();
            Ok(curr_kind)
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
        std::mem::discriminant(&self.current_token.kind) == std::mem::discriminant(&kind)
    }

    /// Consumes the next token.
    pub fn consume(&mut self) {
        self.current_token = self.peek().unwrap();
        self.cursor += 1;
    }

    /// Consumes following tokens until not contained in the kinds vec of TokenKinds.
    pub fn consume_all(&mut self, kinds: Vec<TokenKind>) {
        loop {
            let token = self.peek().unwrap();
            if !kinds.contains(&token.kind) {
                break
            }
            self.current_token = token;
            self.cursor += 1;
        }
    }

    /// Take a look at next token without consuming.
    pub fn peek(&mut self) -> Option<Token<'a>> {
        if self.cursor >= self.tokens.len() {
            None
        } else {
            Some(*self.tokens.get(self.cursor + 1).unwrap())
        }
    }

    /// Take a look at the previous token.
    pub fn peek_behind(&self) -> Option<Token<'a>> {
        if self.cursor == 0 || self.cursor > self.tokens.len() {
            None
        } else {
            Some(*self.tokens.get(self.cursor - 1).unwrap())
        }
    }

    /// Parse a statement.
    fn parse_definition(&mut self) -> Result<(), ParserError> {
        // first token should be keyword "#define"
        self.match_kind(TokenKind::Define)?;
        // match to fucntion, constant, macro, or event
        match self.current_token.kind {
            TokenKind::Function => {
                let _function_definition = self.parse_function().unwrap();
                Ok(())
            }
            TokenKind::Event => {
                let _event_definition = self.parse_event().unwrap();
                Ok(())
            }
            TokenKind::Constant => self.parse_constant(),
            TokenKind::Macro => {
                let _ = self.parse_macro().unwrap();
                Ok(())
            }
            _ => {
                println!(
                    "Invalid definition. Must be a function, event, constant, or macro. Got: {}",
                    self.current_token.kind
                );
                return Err(ParserError::InvalidDefinition)
            }
        }?;
        Ok(())
    }

    /// Parses a function.
    /// Adheres to https://github.com/huff-language/huffc/blob/master/src/parser/high-level.ts#L87-L111
    pub fn parse_function(&mut self) -> Result<Function<'a>, ParserError> {
        // the first token should be of `TokenKind::Function`
        self.match_kind(TokenKind::Function)?;
        // function name should be next
        self.match_kind(TokenKind::Ident("x"))?;
        let tok = self.peek_behind().unwrap().kind;
        let name: &'a str = match tok {
            TokenKind::Ident(fn_name) => fn_name,
            _ => {
                println!("Function name should be of kind Ident. Got: {}", tok);
                return Err(ParserError::InvalidName)
            }
        };

        // function inputs should be next
        let inputs: Vec<String> = self.parse_args(false, true)?;
        // function type should be next
        let fn_type = match self.current_token.kind {
            TokenKind::View => FunctionType::View,
            TokenKind::Pure => FunctionType::Pure,
            TokenKind::Payable => FunctionType::Payable,
            TokenKind::NonPayable => FunctionType::NonPayable,
            _ => return Err(ParserError::UnexpectedType),
        };
        // consume the function type
        self.consume();

        // next token should be of `TokenKind::Returns`
        self.match_kind(TokenKind::Returns)?;
        // function outputs should be next
        let outputs: Vec<String> = self.parse_args(false, true)?;

        Ok(Function { name, inputs, fn_type, outputs })
    }

    /// Parse an event.
    pub fn parse_event(&mut self) -> Result<Event<'a>, ParserError> {
        // The event should start with `TokenKind::Event`
        self.match_kind(TokenKind::Event)?;

        // Parse the event name
        self.match_kind(TokenKind::Ident("x"))?;
        let tok = self.peek_behind().unwrap().kind;

        let name: &'a str = match tok {
            TokenKind::Ident(event_name) => event_name,
            _ => {
                println!("Event name must be of kind Ident. Got: {}", tok);
                return Err(ParserError::InvalidName)
            }
        };

        // Parse the event's parameters
        let parameters: Vec<String> = self.parse_args(true, true)?;

        Ok(Event { name, parameters })
    }

    /// Parse a constant.
    pub fn parse_constant(&mut self) -> Result<(), ParserError> {
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
        self.match_kind(TokenKind::Macro)?;
        let macro_name: String = self.match_kind(TokenKind::Ident("MACRO_NAME"))?.to_string();

        let macro_arguments: Vec<String> = self.parse_args(true, false)?;
        self.match_kind(TokenKind::Assign)?;
        self.match_kind(TokenKind::Takes)?;
        let macro_takes: usize = self.parse_single_arg()?;
        self.match_kind(TokenKind::Returns)?;
        let macro_returns: usize = self.parse_single_arg()?;
        let macro_statements: Vec<Statement<'static>> = self.parse_body()?;

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
                TokenKind::Ident("MACRO_NAME") => {
                    let _literals = self.parse_macro_call();
                    //statements.push(Statement::MacroInvocation("aa": []));
                }
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
    pub fn parse_args(
        &mut self,
        select_name: bool,
        select_type: bool,
    ) -> Result<Vec<String>, ParserError> {
        let mut args: Vec<String> = Vec::new();
        self.match_kind(TokenKind::OpenParen)?;
        while !self.check(TokenKind::CloseParen) {
            // type comes first
            // TODO: match against TokenKind dedicated to EVM Types (uint256, bytes, ...)
            if select_type {
                args.push(self.match_kind(TokenKind::Ident("EVMType"))?.to_string());
            };
            // naming is optional
            // TODO: Are parameter names allowed in Huff? I can't find any examples of it, unless
            // TODO: this is intended to be a new feature. -vex
            // TODO: add name of arg to args vector
            if select_name && self.check(TokenKind::Ident("x")) {
                let _arg_name = self.match_kind(TokenKind::Ident("x"))?.to_string();
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

    /// Parses the following : (x)
    pub fn parse_single_arg(&mut self) -> Result<usize, ParserError> {
        self.match_kind(TokenKind::OpenParen)?;
        let num_token = self.match_kind(TokenKind::Num(0))?;
        let value: usize = match num_token {
            TokenKind::Num(value) => value,
            _ => return Err(ParserError::InvalidArgs), /* Should never reach this code,
                                                        * `match_kind` will throw an error if the
                                                        * token kind isn't a `Num`. */
        };
        self.match_kind(TokenKind::CloseParen)?;
        Ok(value)
    }

    /// Parse call to a macro.
    pub fn parse_macro_call(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::Ident("MACRO_NAME"))?;
        self.parse_macro_call_args()?;
        Ok(())
    }

    /// Parse the arguments of a macro call.
    pub fn parse_macro_call_args(&mut self) -> Result<(), ParserError> {
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

    /// Parses a constant push.
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
