use huff_utils::{
    token::{Token, TokenKind},
};
use std::mem::discriminant;

#[derive(Debug)]
pub enum ParserError {
    SyntaxError,
}

pub struct Parser<'a> {
    // Vector of the tokens
    pub tokens: Vec<Token<'a>>,
    // Current position
    pub pos: usize,
    pub current_token: Token<'a>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        let initial_token = tokens.get(0).unwrap().clone();
        Self {
            tokens: tokens,
            pos: 0,
            current_token: initial_token
        }
    }

    pub fn parse(&mut self) -> Result<(), ParserError> {
        // remove all whitespaces and newlines first
        self.tokens.retain(|&token| {
            match token.kind {
                TokenKind::Whitespace => false,
                TokenKind::Newline => false,
                _ => true
            }
        });
        while !self.check(TokenKind::Eof) {
            self.parse_statement()?;
        }
        Ok(())
    }

    /*
        Match current token to a type.
    */
    pub fn match_kind(&mut self, kind: TokenKind) -> Result<(), ParserError> {
        // if match, consume token
        // if not, return error and stop parsing
        if std::mem::discriminant(&mut self.current_token.kind) == std::mem::discriminant(&kind) {
            self.consume();
            Ok(())
        } else {
            println!("Expected current token of kind {} to match {}", self.current_token.kind, kind);
            Err(ParserError::SyntaxError)
        }
    }


    /*
        Check the current token's type against the given type.
    */
    pub fn check(&mut self, kind: TokenKind) -> bool {
        // check if current token is of type kind
        std::mem::discriminant(&mut self.current_token.kind) == std::mem::discriminant(&kind)
    }

    /*
        Consumes the next token.
    */
    pub fn consume(&mut self) {
        self.current_token = self.peek();
        self.pos += 1;
    }

    /*
        Take a look at next token without consuming.
    */
    pub fn peek(&mut self) -> Token<'a> {
        self.tokens.get(self.pos+1).unwrap().clone()
    }

    // -----------------------------------------------------------------------
    // PARSING LOGIC
    // -----------------------------------------------------------------------
    
    /*
        Parse a statement.
    */
    fn parse_statement(&mut self) -> Result<(), ParserError> {
        // first token should be keyword "#define"
        self.match_kind(TokenKind::Define)?;
        // match to fucntion, constant or macro
        match self.current_token.kind {
            TokenKind::Function => self.parse_function(),
            TokenKind::Constant => self.parse_constant(),
            TokenKind::Macro => self.parse_macro(),
            _ => Err(ParserError::SyntaxError)
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
        
        self.match_kind(TokenKind::FuncType)?; // view, payable or nonpayable
        
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
            },
            _ => Err(ParserError::SyntaxError)
        }
    }

    /*
        Parse a macro.

        It should parse the following : macro MACRO_NAME(args...) = takes (x) returns (n) {...}
    */
    fn parse_macro(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::Macro)?;
        self.match_kind(TokenKind::Ident("MACRO_NAME"))?;
        self.parse_args(true)?;
        self.match_kind(TokenKind::Assign)?;
        self.match_kind(TokenKind::Takes)?;
        self.parse_single_arg()?;
        self.match_kind(TokenKind::Returns)?;
        self.parse_single_arg()?;
        self.parse_body()?;
        Ok(())
    }

    /*
        Parse the body of a macro.

        Only HEX, OPCODES, labels and MACRO calls should be authorized.
    */
    fn parse_body(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::OpenBrace)?;
        while !self.check(TokenKind::CloseBrace) {
            match self.current_token.kind {
                TokenKind::Literal(_) => self.consume(),
                TokenKind::Opcode => self.consume(),
                TokenKind::Label(_) => self.consume(),
                TokenKind::Ident("MACRO_NAME") => self.parse_macro_call()?,
                TokenKind::OpenBracket => self.parse_constant_push()?,
                _ => return Err(ParserError::SyntaxError)
            };
        };
        // consume close brace
        self.match_kind(TokenKind::CloseBrace)?;
        Ok(())
    }

    /*
        Parse new lines.
    */
    fn parse_newline(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::Newline)?;
        while self.check(TokenKind::Newline) {
            self.consume();
        }
        Ok(())
    }

    /*
        Parse arguments, can be typed or not. Between parenthesis.
        Works for both inputs and outputs.

        It should parse the following : (uint256 a, bool b, ...)
    */
    fn parse_args(&mut self, name_only: bool) -> Result<(), ParserError> {
        self.match_kind(TokenKind::OpenParen)?;
        while !self.check(TokenKind::CloseParen) {
            // type comes first
            if name_only {self.match_kind(TokenKind::Type)?};
            // naming is optional
            if self.check(TokenKind::Ident("x")) {
                self.match_kind(TokenKind::Ident("x"))?;
            }
            // multiple args possible
            if self.check(TokenKind::Comma) {
                self.consume();
            }
        }
        // consume close parenthesis
        self.match_kind(TokenKind::CloseParen)?;
        Ok(())
    }

    /*
        Parses the following : (x)
    */
    fn parse_single_arg(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::OpenParen)?;
        self.match_kind(TokenKind::Num(0))?;
        self.match_kind(TokenKind::CloseParen)?;
        Ok(())
    }

    /*
        Parse call to a macro.
    */
    fn parse_macro_call(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::Ident("MACRO_NAME"))?;
        self.parse_macro_call_args()?;
        Ok(())
    }

    /*
        Parse the arguments of a macro call.
    */
    fn parse_macro_call_args(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::OpenParen)?;
        while !self.check(TokenKind::CloseParen) {
            // We can pass either directly hex values or labels (without the ":")
            match self.current_token.kind {
                TokenKind::Literal(_) | TokenKind::Ident(_) => self.consume(),
                _ => return Err(ParserError::SyntaxError)
            }
            if self.check(TokenKind::Comma) {
                self.consume();
            }
        }
        // consume close parenthesis
        self.consume();
        Ok(())
    }

    fn parse_constant_push(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::OpenBracket)?;
        self.match_kind(TokenKind::Ident("CONSTANT"))?;
        self.match_kind(TokenKind::CloseBracket)?;
        Ok(())
    }

    /*
        Parses whitespaces and newlines until none are left.
    */
    fn parse_nl_or_whitespace(&mut self) -> Result<(), ParserError> {
        while self.check(TokenKind::Newline) || self.check(TokenKind::Whitespace) {
            self.consume();
        }
        Ok(())
    } 
}