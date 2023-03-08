/// Defines a context in which the lexing happens.
/// Allows to differientate between EVM types and opcodes that can either
/// be identical or the latter being a substring of the former (example : bytes32 and byte)
#[derive(Debug, PartialEq, Eq)]
pub enum Context {
    /// global context
    Global,
    /// Macro definition context
    MacroDefinition,
    /// Macro's body context
    MacroBody,
    /// Macro's argument context (definition or being called)
    MacroArgs,
    /// ABI context
    Abi,
    /// Lexing args of functions inputs/outputs and events
    AbiArgs,
    /// constant context
    Constant,
    /// Code table context
    CodeTableBody,
}

/// ## Lexer
///
/// The lexer encapsulated in a struct.
pub struct Lexer<'a> {
    /// The source code as peekable chars.
    /// WARN: SHOULD NEVER BE MODIFIED!
    pub reference_chars: Peekable<Chars<'a>>,
    position: Position,
    done: bool,
    /// Current context.
    pub context: Context,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Lexer {
            // We zip with the character index here to ensure the first char has index 0
            char_iter: source.chars().zip(0..).peekable(),
            position: 0,
            done: false,
        }
    }

    /// Consumes the next character
    pub fn consume(&mut self) -> Option<char> {
        self.chars.next().map(|x| {
            self.position += 1;
            x
        })
    }

    /// Peeks at the next char. Does not iterate the cursor
    fn peek_char(&mut self) -> Option<char> {
        self.char_iter.peek().map(|(c, _)| *c)
    }

    /// Dynamically consumes characters based on filters
    pub fn dyn_consume(&mut self, f: impl Fn(&char) -> bool + Copy) {
        while self.peek().map(|x| f(&x)).unwrap_or(false) {
            self.consume();
        }
    }

    fn next_token(&mut self) -> SpannedTokenResult {
        match self.consume() {

        }
    }


}

impl<'a> Iterator for Lexer<'a> {
    type Item = SpannedTokenResult;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            Some(self.next_token())
        }
    }
}