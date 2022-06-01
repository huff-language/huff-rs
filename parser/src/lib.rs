use utils::{
    token::{Token, TokenKind},
};
use std::mem::discriminant;

enum ParserError {
    SyntaxError,
}

struct Parser<'a> {
    // Vector of the tokens
    tokens: Vec<Token<'a>>,
    // Current position
    pos: usize,
    current_token: Token<'a>
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token<'a>>) -> Self {
        let initial_token = tokens.get(0).unwrap().clone();
        Self {
            tokens: tokens,
            pos: 0,
            current_token: initial_token
        }
    }

    fn parse(&mut self) -> Result<(), ParserError> {
        while !self.check(TokenKind::EOF) {
            self.parse_statement();
        }
        Ok(())
    }

    /*
        Match current token to a type.
    */
    fn match_kind(&mut self, kind: TokenKind) -> Result<(), ParserError> {
        // if match, consume token
        // if not, return error and stop parsing
        if std::mem::discriminant(&mut self.current_token.kind) == std::mem::discriminant(&kind) {
            self.consume();
            Ok(())
        } else {
            Err(ParserError::SyntaxError)
        }
    }


    /*
        Check the current token's type against the given type.
    */
    fn check(&mut self, kind: TokenKind) -> bool {
        // check if current token is of type kind
        self.current_token.kind == kind
    }

    /*
        Consumes the next token.
    */
    fn consume(&mut self) {
        self.current_token = self.peek();
        self.pos += 1;
    }

    /*
        Take a look at next token without consuming.
    */
    fn peek(&mut self) -> Token<'a> {
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
        self.parse_newline();
        Ok(())
    }

    /*
        Parse a function.
    */
    fn parse_function(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::Function)?;
        // function name should be next
        self.match_kind(TokenKind::Ident("x"))?;
        self.match_kind(TokenKind::OpenParen)?;
        self.match_kind(TokenKind::NamedArgs)?;
        self.match_kind(TokenKind::CloseParen)?;
        self.match_kind(TokenKind::FuncType)?; // view, payable or nonpayable
        self.match_kind(TokenKind::Returns)?;
        self.match_kind(TokenKind::NamedArgs)?;
        Ok(())
    }

    /*
        Parse a constant.
    */
    fn parse_constant(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::Constant)?;
        self.match_kind(TokenKind::Ident("x"))?;
        self.match_kind(TokenKind::Equal)?;
        match self.current_token.kind {
            TokenKind::FreeStoragePointer | TokenKind::Hex => {
                self.consume();
                Ok(())
            },
            _ => Err(ParserError::SyntaxError)
        }
    }

    fn parse_macro(&mut self) -> Result<(), ParserError> {
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
        Parse function (interface) arguments, can be typed or not. Between parenthesis.
        Works for both inputs and outputs.
    */
    fn parse_function_args(&mut self) -> Result<(), ParserError> {
        self.match_kind(TokenKind::OpenParen)?;
        while !self.check(TokenKind::CloseParen) {
            // type comes first
            self.match_kind(TokenKind::Type)?;
            // naming is optional
            if self.check(TokenKind::Ident("x")) {
                self.match_kind(TokenKind::Ident("x"))?;
            }
            // multiple args possible
            if self.check(TokenKind::Comma) {
                self.consume();
            }
        }
        self.match_kind(TokenKind::NamedArgs)?;
        Ok(())
    }
}