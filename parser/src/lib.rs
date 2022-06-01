
// to be replaced with actual Token type from the lexer
#[derive(Debug, PartialEq, Eq, Clone)]
enum TokenType {
    NEWLINE,
    DEFINE,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACKET,
    RIGHT_BRACKET,
    COMMA,
    TAKES,
    RETURNS,
    EQUAL,
    FUNC_TYPE,
    FUNCTION,
    CONSTANT,
    MACRO,
    HEX,
    OPCODE,
    EOF,
    IDENT,
    TYPE,
    LABEL,
    ARGS,
    TYPED_ARGS,
    BODY,
    MACRO,
    CONSTANT,
    FUNCTION,
    PATH,
    INCLUDE,
    STATEMENT,
    PROGRAM
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Token {
    ttype : TokenType,
}

enum ParserError {
    SyntaxError,
}

struct Parser {
    // Vector of the tokens
    tokens: Vec<Token>,
    // Current position
    pos: usize,
    current_token: Token
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            pos: 0,
            current_token: tokens.get(0).unwrap().clone()
        }
    }

    fn parse(&self) -> Result<(), ParserError> {
        while !self.check(TokenType::EOF) {
            self.statement();
        }
        Ok(())
    }

    /*
        Match current token to a type.
    */
    fn match_ttype(&self, ttype: TokenType) -> Result<(), ParserError> {
        // if match, consume token
        // if not, return error and stop parsing
        if let self.current_token.ttype = ttype {
            self.consume();
            Ok(())
        } else {
            Err(ParserError::SyntaxError)
        }
    }


    /*
        Check the current token's type against the given type.
    */
    fn check(&self, ttype: TokenType) -> bool {
        // check if current token is of type ttype
        self.current_token.ttype == ttype
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
    fn peek(&self) -> Token {
        Token {
            ttype: TokenType::EOF
        }
    }

    // -----------------------------------------------------------------------
    // PARSING LOGIC
    // -----------------------------------------------------------------------
    
    /*
        Parse a statement.
    */
    fn statement(&self) -> Result<(), ParserError> {
        // first token should be keyword "#define"
        self.match_ttype(TokenType::DEFINE)?;
        // match to fucntion, constant or macro
        match self.current_token.ttype {
            TokenType::FUNCTION => self.function(),
            TokenType::CONSTANT => self.constant(),
            TokenType::MACRO => self.macro(),
            _ => Err(ParserError::SyntaxError)
        }
        self.newline();
    }

    /*
        Parse a function.
    */
    fn function(&self) -> Result<(), ParserError> {
        self.match_ttype(TokenType::FUNCTION)?;
        // function name should be next
        self.match_ttype(TokenType::IDENT)?;
        self.match_ttype(TokenType::LEFT_PAREN)?;
        self.match_ttype(TokenType::TYPED_ARGS)?;
        self.match_ttype(TokenType::RIGHT_PAREN)?;
        self.match_ttype(TokenType::FUNC_TYPE)?; // view, payable or nonpayable
        self.match_ttype(TokenType::RETURNS)?;
        self.match_ttype(TokenType::TYPED_ARGS)?;
        Ok(())
    }

    /*
        Parse new lines.
    */
    fn newline(&self) -> Result<(), ParserError> {
        self.match_ttype(TokenType::NEWLINE)?;
        while self.check(TokenType::NEWLINE) {
            self.consume();
        }
        Ok(())
    }
}