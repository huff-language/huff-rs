#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

use huff_utils::{
    ast::*,
    error::ParserError,
    token::{Token, TokenKind},
    types::*,
};
use std::path::Path;
use tiny_keccak::{Hasher, Keccak};

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

    /// Resets the current token and cursor to the first token in the parser's token vec
    ///
    /// PANICS if the tokens vec is empty!
    pub fn reset(&mut self) {
        self.current_token = *self.tokens.get(0).unwrap();
        self.cursor = 0;
    }

    /// Parse
    pub fn parse(&mut self) -> Result<Contract<'a>, ParserError> {
        // Remove all whitespaces, newlines, and comments first
        self.tokens
            .retain(|&token| !matches!(token.kind, TokenKind::Whitespace | TokenKind::Comment(_)));

        // Reset the initial token
        self.reset();

        // Initialize an empty Contract
        let mut contract: Contract<'a> = Contract::<'a>::default();

        // First iterate over imports
        while !self.check(TokenKind::Eof) && !self.check(TokenKind::Define) {
            contract.imports.push(self.parse_imports()?);
        }

        // Iterate over tokens and construct the Contract aka AST
        while !self.check(TokenKind::Eof) {
            // first token should be keyword "#define"
            self.match_kind(TokenKind::Define)?;

            // match to fucntion, constant, macro, or event
            match self.current_token.kind {
                TokenKind::Function => {
                    contract.functions.push(self.parse_function()?);
                }
                TokenKind::Event => {
                    contract.events.push(self.parse_event()?);
                }
                TokenKind::Constant => {
                    contract.constants.push(self.parse_constant()?);
                }
                TokenKind::Macro => {
                    contract.macros.push(self.parse_macro()?);
                }
                _ => {
                    tracing::error!(
                        "Invalid definition. Must be a function, event, constant, or macro. Got: {}",
                        self.current_token.kind
                    );
                    return Err(ParserError::InvalidDefinition)
                }
            };
        }

        Ok(contract)
    }

    /// Parses Contract Imports
    pub fn parse_imports(&mut self) -> Result<FilePath<'a>, ParserError> {
        // First token should be keyword "#include"
        self.match_kind(TokenKind::Include)?;

        // Then let's grab and validate the file path
        self.match_kind(TokenKind::Str("x"))?;
        let tok = self.peek_behind().unwrap().kind;
        let path: &'a Path = Path::new(match tok {
            TokenKind::Str(file_path) => file_path,
            _ => {
                println!("Invalid import path string. Got: {}", tok);
                return Err(ParserError::InvalidName)
            }
        });

        // Validate that a file @ the path exists
        if !(path.exists() && path.is_file() && path.to_str().unwrap().ends_with(".huff")) {
            println!("Invalid file path. Got: {}", path.to_str().unwrap());
            return Err(ParserError::InvalidImportPath)
        }

        Ok(path)
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
        let inputs: Vec<Argument> = self.parse_args(true, true, false)?;
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
        let outputs: Vec<Argument> = self.parse_args(true, true, false)?;

        let mut signature = [0u8; 4]; // Only keep first 4 bytes
        let mut hasher = Keccak::v256();
        let input_types =
            inputs.iter().map(|i| i.arg_type.as_ref().unwrap().clone()).collect::<Vec<_>>();
        hasher.update(format!("{}({})", name, input_types.join(",")).as_bytes());
        hasher.finalize(&mut signature);

        Ok(Function { name, signature, inputs, fn_type, outputs })
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
        let parameters: Vec<Argument> = self.parse_args(true, true, true)?;

        Ok(Event { name, parameters })
    }

    /// Parse a constant.
    pub fn parse_constant(&mut self) -> Result<ConstantDefinition<'a>, ParserError> {
        // Constant Identifier
        self.match_kind(TokenKind::Constant)?;

        // Parse the constant name
        self.match_kind(TokenKind::Ident("x"))?;
        let tok = self.peek_behind().unwrap().kind;
        let name: &'a str = match tok {
            TokenKind::Ident(event_name) => event_name,
            _ => {
                println!("Event name must be of kind Ident. Got: {}", tok);
                return Err(ParserError::InvalidName)
            }
        };

        // We must assign a value to the constant
        self.match_kind(TokenKind::Assign)?;

        let value: ConstVal = match self.current_token.kind {
            TokenKind::FreeStoragePointer => {
                self.consume();
                ConstVal::FreeStoragePointer(FreeStoragePointer {})
            }
            TokenKind::Literal(l) => {
                self.consume();
                ConstVal::Literal(l)
            }
            _ => {
                println!(
                    "Constant value must be of kind FreeStoragePointer or Literal. Got: {}",
                    self.current_token.kind
                );
                return Err(ParserError::InvalidConstantValue)
            }
        };

        // Return the Constant Definition
        Ok(ConstantDefinition { name, value })
    }

    /// Parses a macro.
    ///
    /// It should parse the following : macro MACRO_NAME(args...) = takes (x) returns (n) {...}
    pub fn parse_macro(&mut self) -> Result<MacroDefinition<'a>, ParserError> {
        self.match_kind(TokenKind::Macro)?;
        let macro_name: String = self.match_kind(TokenKind::Ident("MACRO_NAME"))?.to_string();

        let macro_arguments: Vec<Argument> = self.parse_args(true, false, false)?;
        self.match_kind(TokenKind::Assign)?;
        self.match_kind(TokenKind::Takes)?;
        let macro_takes: usize = self.parse_single_arg()?;
        self.match_kind(TokenKind::Returns)?;
        let macro_returns: usize = self.parse_single_arg()?;
        let macro_statements: Vec<Statement<'a>> = self.parse_body()?;

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
    pub fn parse_body(&mut self) -> Result<Vec<Statement<'a>>, ParserError> {
        let mut statements: Vec<Statement<'a>> = Vec::new();
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
                TokenKind::Ident(ident_str) => {
                    tracing::info!("Found iden string in macro: {}", ident_str);
                    let lit_args = self.parse_macro_call()?;
                    statements.push(Statement::MacroInvocation(MacroInvocation {
                        macro_name: ident_str.to_string(),
                        args: lit_args,
                    }));
                }
                TokenKind::Label(_) => {
                    self.consume();
                }
                TokenKind::OpenBracket => {
                    let constant = self.parse_constant_push()?;
                    statements.push(Statement::Constant(constant));
                }
                TokenKind::LeftAngle => {
                    let arg_call = self.parse_arg_call()?;
                    statements.push(Statement::ArgCall(arg_call));
                }
                _ => {
                    tracing::error!("Invalid Macro Body Token: {:?}", self.current_token);
                    return Err(ParserError::SyntaxError(format!(
                        "Invalid token in macro body: {:?}. Must be of kind Hex, Opcode, Macro, or Label.",
                        self.current_token
                    )))
                }
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
        has_indexed: bool,
    ) -> Result<Vec<Argument>, ParserError> {
        let mut args: Vec<Argument> = Vec::new();
        self.match_kind(TokenKind::OpenParen)?;
        while !self.check(TokenKind::CloseParen) {
            let mut arg = Argument::default();

            // type comes first
            // TODO: match against TokenKind dedicated to EVM Types (uint256, bytes, ...)
            if select_type {
                arg.arg_type = Some(self.parse_arg_type()?.to_string());
                // Check if the argument is indexed
                // TODO: Ensure this can only be done for events- this function is used for
                // TODO: events, functions, and macro arguments.
                if has_indexed && self.check(TokenKind::Indexed) {
                    arg.indexed = true;
                    self.consume(); // consume "indexed" keyword
                }
            }

            // name comes second (is optional)
            if select_name && self.check(TokenKind::Ident("x")) {
                arg.name = Some(self.match_kind(TokenKind::Ident("x"))?.to_string())
            }

            // multiple args possible
            if self.check(TokenKind::Comma) {
                self.consume();
            }

            args.push(arg);
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
    pub fn parse_macro_call(&mut self) -> Result<Vec<MacroArg<'a>>, ParserError> {
        self.match_kind(TokenKind::Ident("MACRO_NAME"))?;
        self.parse_macro_call_args()
    }

    /// Parse the arguments of a macro call.
    pub fn parse_macro_call_args(&mut self) -> Result<Vec<MacroArg<'a>>, ParserError> {
        let mut args = vec![];
        self.match_kind(TokenKind::OpenParen)?;
        while !self.check(TokenKind::CloseParen) {
            // We can pass either directly hex values or labels (without the ":")
            match self.current_token.kind {
                TokenKind::Literal(lit) => {
                    args.push(MacroArg::Literal(lit));
                    self.consume();
                }
                TokenKind::Ident(ident) => {
                    args.push(MacroArg::Ident(ident));
                    self.consume();
                }
                _ => {
                    tracing::error!(
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
        Ok(args)
    }

    /// Parses a constant push.
    pub fn parse_constant_push(&mut self) -> Result<&'a str, ParserError> {
        self.match_kind(TokenKind::OpenBracket)?;
        match self.current_token.kind {
            TokenKind::Ident(const_str) => {
                // Consume the Ident and Validate Close Bracket
                self.consume();
                self.match_kind(TokenKind::CloseBracket)?;
                Ok(const_str)
            }
            _ => Err(ParserError::InvalidConstant),
        }
    }

    /// Parses an argument call.
    ///
    /// ## Examples
    ///
    /// When an argument is called in Huff, it is wrapped in angle brackets like so:
    ///
    /// ```huff
    /// #define macro EXAMPLE_FUNCTION(error) = takes (0) returns (0) {
    ///     <error> jumpi
    /// }
    /// ```
    pub fn parse_arg_call(&mut self) -> Result<&'a str, ParserError> {
        self.match_kind(TokenKind::LeftAngle)?;
        match self.current_token.kind {
            TokenKind::Ident(arg_str) => {
                self.consume();
                self.match_kind(TokenKind::RightAngle)?;
                Ok(arg_str)
            }
            _ => Err(ParserError::InvalidMacroArgs),
        }
    }

    /// Parses whitespaces and newlines until none are left.
    pub fn parse_nl_or_whitespace(&mut self) -> Result<(), ParserError> {
        while self.check(TokenKind::Whitespace) {
            self.consume();
        }
        Ok(())
    }

    /// Parses the type of an argument.
    pub fn parse_arg_type(&mut self) -> Result<TokenKind, ParserError> {
        match self.current_token.kind {
            TokenKind::PrimitiveType(prim) => Ok(self.parse_primitive_type(prim)?),
            TokenKind::ArrayType(prim, _size) => {
                let _ = self.parse_primitive_type(prim);
                Ok(self.match_kind(self.current_token.kind)?)
            }
            _ => Err(ParserError::InvalidArgs),
        }
    }

    /// Parses a primitive EVM type.
    /// Arrays of primitive types are not considered as primitive types themselves.
    pub fn parse_primitive_type(
        &mut self,
        prim: PrimitiveEVMType,
    ) -> Result<TokenKind, ParserError> {
        match prim {
            PrimitiveEVMType::Uint(size) => {
                if !(8..=256).contains(&size) || size % 8 != 0 {
                    return Err(ParserError::InvalidArgs)
                }
                Ok(self.match_kind(self.current_token.kind)?)
            }
            PrimitiveEVMType::Bytes(size) => {
                if !(1..=32).contains(&size) {
                    return Err(ParserError::InvalidArgs)
                }
                Ok(self.match_kind(self.current_token.kind)?)
            }
            PrimitiveEVMType::Bool => Ok(self.match_kind(self.current_token.kind)?),
            PrimitiveEVMType::Address => Ok(self.match_kind(self.current_token.kind)?),
            PrimitiveEVMType::String => Ok(self.match_kind(self.current_token.kind)?),
            PrimitiveEVMType::DynBytes => Ok(self.match_kind(self.current_token.kind)?),
            PrimitiveEVMType::Int(size) => {
                if !(8..=256).contains(&size) || size % 8 != 0 {
                    return Err(ParserError::InvalidArgs)
                }
                let curr_token_kind = self.current_token.kind;
                self.consume();
                Ok(curr_token_kind)
            }
        }
    }
}
