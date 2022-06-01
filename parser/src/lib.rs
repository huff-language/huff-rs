use utils::{
    token::{Token, TokenKind},
};
// // to be replaced with actual Token type from the lexer
// #[derive(Debug, PartialEq, Eq, Clone)]
// enum TokenKind {
//     NEWLINE,
//     DEFINE,
//     LEFT_PAREN,
//     RIGHT_PAREN,
//     LEFT_BRACKET,
//     RIGHT_BRACKET,
//     COMMA,
//     TAKES,
//     RETURNS,
//     EQUAL,
//     FUNC_TYPE,
//     FUNCTION,
//     CONSTANT,
//     FREE_STORAGE_POINTER,
//     MACRO,
//     HEX,
//     OPCODE,
//     EOF,
//     IDENT,
//     TYPE,
//     LABEL,
//     ARGS,
//     TYPED_ARGS,
//     BODY,
//     PATH,
//     INCLUDE,
//     STATEMENT,
//     PROGRAM
// }

// #[derive(Debug, PartialEq, Eq, Clone)]
// struct Token {
//     ttype : TokenKind,
// }

//---------------------------------

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
        Self {
            tokens: tokens,
            pos: 0,
            current_token: tokens.get(0).unwrap().clone()
        }
    }

    fn parse(&self) -> Result<(), ParserError> {
        while !self.check(TokenKind::EOF) {
            self.parse_statement();
        }
        Ok(())
    }

    /*
        Match current token to a type.
    */
    fn match_ttype(&self, ttype: TokenKind) -> Result<(), ParserError> {
        // if match, consume token
        // if not, return error and stop parsing
        if let self.current_token.kind = ttype {
            self.consume();
            Ok(())
        } else {
            Err(ParserError::SyntaxError)
        }
    }


    /*
        Check the current token's type against the given type.
    */
    fn check(&self, kind: TokenKind) -> bool {
        // check if current token is of type ttype
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
    fn peek(&self) -> Token<'a> {
        self.tokens.get(self.pos+1).unwrap().clone()
    }

    // -----------------------------------------------------------------------
    // PARSING LOGIC
    // -----------------------------------------------------------------------
    
    /*
        Parse a statement.
    */
    fn statement(&self) -> Result<(), ParserError> {
        // first token should be keyword "#define"
        self.match_ttype(TokenKind::Define)?;
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
    fn parse_function(&self) -> Result<(), ParserError> {
        self.match_ttype(TokenKind::Function)?;
        // function name should be next
        self.match_ttype(TokenKind::Ident)?;
        self.match_ttype(TokenKind::OpenParen)?;
        self.match_ttype(TokenKind::NamedArgs)?;
        self.match_ttype(TokenKind::CloseParen)?;
        self.match_ttype(TokenKind::FuncType)?; // view, payable or nonpayable
        self.match_ttype(TokenKind::Returns)?;
        self.match_ttype(TokenKind::NamedArgs)?;
        Ok(())
    }

    /*
        Parse a constant.
    */
    fn parse_constant(&self) -> Result<(), ParserError> {
        self.match_ttype(TokenKind::Constant)?;
        self.match_ttype(TokenKind::Ident(ref value))?;
        self.match_ttype(TokenKind::Equal)?;
        match self.current_token.kind {
            TokenKind::FreeStoragePointer | TokenKind::Hex => {
                self.consume();
                Ok(())
            },
            _ => Err(ParserError::SyntaxError)
        }
    }

    fn parse_macro(&self) -> Result<(), ParserError> {
        Ok(())
    }

    /*
        Parse new lines.
    */
    fn parse_newline(&self) -> Result<(), ParserError> {
        self.match_ttype(TokenKind::Newline)?;
        while self.check(TokenKind::Newline) {
            self.consume();
        }
        Ok(())
    }

    /*
        Parse function (interface) arguments, can be typed or not. Between parenthesis.
        Works for both inputs and outputs.
    */
    fn parse_function_args(&self) -> Result<(), ParserError> {
        self.match_ttype(TokenKind::OpenParen)?;
        while !self.check(TokenKind::CloseParen) {
            // type comes first
            self.match_ttype(TokenKind::Type)?;
            // naming is optional
            if self.check(TokenKind::Ident(ref value)) {
                self.match_ttype(TokenKind::Ident(ref value))?;
            }
            // multiple args possible
            if self.check(TokenKind::Comma) {
                self.consume();
            }
        }
        self.match_ttype(TokenKind::NamedArgs)?;
        Ok(())
    }
}