use huffr_utils::token::Token;

type Literal = [u8; 32];

struct MacroDefinition<'a> {
    name: String,
    arguments: Vec<String>,
    statements: Vec<Statement<'a>>,
    takes: usize,
    returns: usize,
}

struct MacroInvocation<'a> {
    macro_name: String,
    args: Vec<&'a Literal>,
}

enum Statement<'a> {
    Literal(Literal),
    Opcode,
    MacroInvocation(MacroInvocation<'a>),
}

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
