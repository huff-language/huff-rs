use huffr_utils::token::Token;

struct Parser<'a> {
    cursor: usize,
    tokens: Vec<Token<'a>>,
}

impl Parser<'_> {
    fn new<'a>(tokens: Vec<Token<'static>>) -> Self {
        Self { cursor: 0, tokens }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.cursor).unwrap()
    }

    fn peek_n(&self, n: usize) -> &Token {
        self.tokens.get(self.cursor + n).unwrap()
    }
}
