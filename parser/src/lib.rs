
// to be replaced with actual Token type from the lexer
enum TokenType {
    IDENT,
    DEFINE
}
struct Token;

enum ParserError {
    SyntaxError,
}

struct Parser<'a> {
    // Vector of the tokens
    tokens: Vec<Token>,
    // Current position
    pos: usize,
    current_token: Option<&'a Token>
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            pos: 0,
            current_token: None
        }
    }

    fn parse() -> Result<(), ParserError> {
        Ok(())
    }

    /*
        Match current token to a type.
    */
    fn match_ttype(&self, ttype: TokenType) -> Result<(), ParserError> {
        // if match, consume token
        // if not, return error and stop parsing
        Ok(())
    }

    fn consume(&mut self) {
        //self.current_token = self.peek();
        //self.pos += 1;
    }

    /*
        Take a look at next token without consuming.
    */
    fn peek(&self) -> Token {
        Token {}
    }
}