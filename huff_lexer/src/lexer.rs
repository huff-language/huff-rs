use huff_utils::prelude::*;
use regex::Regex;
use std::{
    iter::Peekable,
    str::Chars,
};
/* hiehgsebgoiesgoiseg */
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
pub struct LexerNew<'a> {
    /// The source code as peekable chars.
    /// WARN: SHOULD NEVER BE MODIFIED!
    pub chars: Peekable<Chars<'a>>,
    position: u32,
    /// The previous lexed Token.
    /// NOTE: Cannot be a whitespace.
    pub lookback: Option<Token>,
    eof: bool,
    /// Current context.
    pub context: Context,
}

pub type TokenResult<'a> = Result<Token, LexicalError<'a>>;

impl<'a> LexerNew<'a> {
    fn new(source: &'a str) -> Self {
        LexerNew {
            // We zip with the character index here to ensure the first char has index 0
            chars: source.chars().peekable(),
            position: 0,
            lookback: None,
            eof: false,
            context: Context::Global,
        }
    }

    /// Consumes the next character
    pub fn consume(&mut self) -> Option<char> {
        self.chars.next().map(|x| {
            self.position += 1;
            x
        })
    }

    /// Try to peek at the next character from the source
    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// Consume characters until a sequence matches
    // pub fn seq_consume(&mut self, word: &str) {
    //     let mut current_pos = self.current_span().start;
    //     while self.peek().is_some() {
    //         let peeked = self.peek_n_chars_from(word.len(), current_pos);
    //         if word == peeked {
    //             break
    //         }
    //         self.consume();
    //         current_pos += 1;
    //     }
    // }
    
    /// Dynamically consumes characters based on filters
    pub fn dyn_consume(&mut self, f: impl Fn(&char) -> bool + Copy) {
        while self.peek().map(|x| f(&x)).unwrap_or(false) {
            self.consume();
        }
    }

    fn next_token(&mut self) -> TokenResult {
        // let start = self.position;
        if let Some(ch) = self.consume() {
            let token = match ch {
                '/' => {


                    if let Some(ch2) = self.peek() {
                        match ch2 {
                            '/' => {
                                // Consume until newline
                                let (comment_string, start, end) = self.eat_while(None, |c| c != '\n');
                                Ok(TokenKind::Comment(comment_string).into_span(start, end))
                            }
                            '*' => {
                                self.consume();
                                let mut depth = 1usize;
                                while let Some(c) = self.consume() {
                                    match c {
                                        '/' if self.peek() == Some('*') => {
                                            self.consume();
                                            depth += 1;
                                        }
                                        '*' if self.peek() == Some('/') => {
                                            self.consume();
                                            depth -= 1;
                                            if depth == 0 {

                                                break;
                                            }
                                        }
                                        _ => (),
                                            

                                    }
                                }


                                let (comment_string, start, end) = self.eat_while(None, |c| c != '*' && self.peek() != Some('/'));

                                Ok(TokenKind::Comment(comment_string).into_span(start, end))
                            }
                            _ => self.single_char_token(TokenKind::Div)
                        }
                    }
                    else {
                        self.single_char_token(TokenKind::Div)
                    }
                }

                // # keywords
                '#' => {
                    let (word, start, end) = self.eat_while(Some(ch), |ch| {
                        ch.is_ascii_alphabetic()
                    });

                    let mut found_kind: Option<TokenKind> = None;

                    let keys = [TokenKind::Define, TokenKind::Include];
                    for kind in keys.into_iter() {
                        let key = kind.to_string();
                        let peeked = word.clone();

                        if key == peeked {
                            found_kind = Some(kind);
                            break
                        }
                    }

                    if let Some(kind) = &found_kind {
                        Ok(kind.clone().into_span(start, end))
                    } else if self.context == Context::Global && self.peek().unwrap() == '[' {
                        Ok(TokenKind::Pound.into_single_span(self.position))
                    } else {
                        // Otherwise we don't support # prefixed indentifiers
                        tracing::error!(target: "lexer", "INVALID '#' CHARACTER USAGE");
                        return Err(LexicalError::new(
                            LexicalErrorKind::InvalidCharacter('#'),
                            Span { start: self.position as usize, end: self.position as usize, file: None },
                        ))
                    }
                }
                // Alphabetical characters
                ch if ch.is_alphabetic() || ch.eq(&'_') => {
                    let (word, start, end) = self.eat_while(Some(ch), |c| {
                        c.is_alphanumeric() || c == '_'
                    });

                    let mut found_kind: Option<TokenKind> = None;
                    let keys = [
                        TokenKind::Macro,
                        TokenKind::Fn,
                        TokenKind::Test,
                        TokenKind::Function,
                        TokenKind::Constant,
                        TokenKind::Error,
                        TokenKind::Takes,
                        TokenKind::Returns,
                        TokenKind::Event,
                        TokenKind::NonPayable,
                        TokenKind::Payable,
                        TokenKind::Indexed,
                        TokenKind::View,
                        TokenKind::Pure,
                        // First check for packed jump table
                        TokenKind::JumpTablePacked,
                        // Match with jump table if not
                        TokenKind::JumpTable,
                        TokenKind::CodeTable,
                    ];
                    for kind in keys.into_iter() {
                        if self.context == Context::MacroBody {
                            break
                        }
                        let key = kind.to_string();
                        let peeked = word.clone();

                        if key == peeked {
                            found_kind = Some(kind);
                            break
                        }
                    }

                    // Check to see if the found kind is, in fact, a keyword and not the name of
                    // a function. If it is, set `found_kind` to `None` so that it is set to a
                    // `TokenKind::Ident` in the following control flow.
                    if !self.check_keyword_rules(&found_kind) {
                        found_kind = None;
                    }

                    if let Some(kind) = &found_kind {
                        match kind {
                            TokenKind::Macro | TokenKind::Fn | TokenKind::Test => {
                                self.context = Context::MacroDefinition
                            }
                            TokenKind::Function | TokenKind::Event | TokenKind::Error => {
                                self.context = Context::Abi
                            }
                            TokenKind::Constant => self.context = Context::Constant,
                            TokenKind::CodeTable => self.context = Context::CodeTableBody,
                            _ => (),
                        }
                    }

                    // Check for free storage pointer builtin
                    let fsp = "FREE_STORAGE_POINTER";
                    if fsp == word {
                        // Consume the parenthesis following the FREE_STORAGE_POINTER
                        // Note: This will consume `FREE_STORAGE_POINTER)` or
                        // `FREE_STORAGE_POINTER(` as well
                        if let Some('(') = self.peek() {
                            self.consume();
                        }
                        if let Some(')') = self.peek() {
                            self.consume();
                        }
                        found_kind = Some(TokenKind::FreeStoragePointer);
                    }

                    if let Some(':') = self.peek() {
                        found_kind = Some(TokenKind::Label(word));
                        self.consume();
                    }

                    


                    if !(self.context != Context::MacroBody || found_kind.is_some()) {
                        if let Some(o) = OPCODES_MAP.get(&word) {
                            found_kind = Some(TokenKind::Opcode(o.to_owned()));
                        }
                    }

                    if let Some(kind) = &found_kind {
                        return Ok(kind.clone().into_span(start, end))
                    } 

                    // let slice = self.slice();
                    let kind = if self.context == Context::MacroBody &&
                        BuiltinFunctionKind::try_from(&word).is_ok() {
                        TokenKind::BuiltinFunction(word)
                    } else {
                        TokenKind::Ident(word)
                    };

                    Ok(kind.into_span(start, end))
                }
                // If it's the start of a hex literal
                ch if ch == '0' && self.peek().unwrap() == 'x' => {
                    self.eat_hex_digit(ch)
                }
                '=' => self.single_char_token(TokenKind::Assign),
                '(' => {
                    match self.context {
                        Context::Abi => self.context = Context::AbiArgs,
                        Context::MacroBody => self.context = Context::MacroArgs,
                        _ => {}
                    }
                    self.single_char_token(TokenKind::OpenParen)
                }
                ')' => {
                    match self.context {
                        Context::AbiArgs => self.context = Context::Abi,
                        Context::MacroArgs => self.context = Context::MacroBody,
                        _ => {}
                    }
                    self.single_char_token(TokenKind::CloseParen)
                }
                '[' => self.single_char_token(TokenKind::OpenBracket),
                ']' => self.single_char_token(TokenKind::CloseBracket),
                '{' => {
                    if self.context == Context::MacroDefinition {
                        self.context = Context::MacroBody;
                    }
                    self.single_char_token(TokenKind::OpenBrace)
                }
                '}' => {
                    if matches!(self.context, Context::MacroBody | Context::CodeTableBody) {
                        self.context = Context::Global;
                    }
                    self.single_char_token(TokenKind::CloseBrace)
                }
                '+' => self.single_char_token(TokenKind::Add),
                '-' => self.single_char_token(TokenKind::Sub),
                '*' => self.single_char_token(TokenKind::Mul),
                '<' => self.single_char_token(TokenKind::LeftAngle),
                '>' => self.single_char_token(TokenKind::LeftAngle),
                // NOTE: TokenKind::Div is lexed further up since it overlaps with comment
                ':' => self.single_char_token(TokenKind::LeftAngle),
                // identifiers
                ',' => self.single_char_token(TokenKind::LeftAngle),
                '0'..='9' => self.eat_digit(ch),
                // Lexes Spaces and Newlines as Whitespace
                ch if ch.is_ascii_whitespace() => {
                    self.eat_whitespace();
                    self.next_token()
                }
                // String literals. String literals can also be wrapped by single quotes
                '"' | '\'' => {
                    Ok(self.eat_string_literal())
                }
                ch => {
                    tracing::error!(target: "lexer", "UNSUPPORTED TOKEN '{}'", ch);
                    return Err(LexicalError::new(
                        LexicalErrorKind::InvalidCharacter(ch),
                        Span { start: self.position as usize, end: self.position as usize, file: None },
                    ))
                }
            }?;

            if token.kind != TokenKind::Whitespace {
                self.lookback = Some(token.clone());
            }

            return Ok(token)
        } else {
            self.eof = true;
            Ok(Token { kind: TokenKind::Eof, span: Span { start: self.position as usize, end: self.position as usize, file: None } } )
        }
    }

    fn single_char_token(&self, token_kind: TokenKind) -> TokenResult {
        Ok(token_kind.into_single_span(self.position))
    }
    
    /// Keeps consuming tokens as long as the predicate is satisfied
    fn eat_while<F: Fn(char) -> bool>(
        &mut self,
        initial_char: Option<char>,
        predicate: F,
    ) -> (String, u32, u32) {
        let start = self.position;

        // This function is only called when we want to continue consuming a character of the same type.
        // For example, we see a digit and we want to consume the whole integer
        // Therefore, the current character which triggered this function will need to be appended
        let mut word = String::new();
        if let Some(init_char) = initial_char {
            word.push(init_char)
        }

        // Keep checking that we are not at the EOF
        while let Some(peek_char) = self.peek() {
            // Then check for the predicate, if predicate matches append char and increment the cursor
            // If not, return word. The next character will be analyzed on the next iteration of next_token,
            // Which will increment the cursor
            if !predicate(peek_char) {
                return (word, start, self.position);
            }
            word.push(peek_char);

            // If we arrive at this point, then the char has been added to the word and we should increment the cursor
            self.consume();
        }

        (word, start, self.position)
    }


    fn eat_digit(&mut self, initial_char: char) -> TokenResult {
        let (integer_str, start, end) = self.eat_while(Some(initial_char), |ch| {
            ch.is_ascii_digit()
        });

        let integer = integer_str.parse().unwrap();

        let integer_token = TokenKind::Num(integer);
        let span = Span { start: start as usize, end: end as usize, file: None };
        Ok(Token { kind: integer_token, span })
    }

    fn eat_hex_digit(&mut self, initial_char: char) -> TokenResult {
        let (integer_str, start, end) = self.eat_while(Some(initial_char), |ch| {
            ch.is_ascii_hexdigit() | (ch == 'x')
        });
        // TODO: check for sure that we have a correct hex string, eg. 0x56 and not 0x56x34

        let kind = if self.context == Context::CodeTableBody {
            // In codetables, the bytecode provided is of arbitrary length. We pass
            // the code as an Ident, and it is appended to the end of the runtime
            // bytecode in codegen.
            TokenKind::Ident(integer_str)
        } else {
            TokenKind::Literal(str_to_bytes32(&integer_str.as_ref()))
        };

        let span = Span { start: start as usize, end: end as usize, file: None };
        Ok(Token { kind, span })
    }

    /// Skips white space. They are not significant in the source language
    fn eat_whitespace(&mut self) {
        self.eat_while(None, |ch| ch.is_whitespace());
    }

    fn eat_string_literal(&mut self) -> Token {
        let (str_literal, start_span, end_span) = self.eat_while(None, |ch| ch != '"');
        let str_literal_token = TokenKind::Str(str_literal);
        self.consume(); // Advance past the closing quote
        str_literal_token.into_span(start_span, end_span)
    }

    // fn eat_alphabetic(&mut self, initial_char: char) -> (String, u32, u32) {
    //     let (word, start, end) = self.eat_while(Some(initial_char), |ch| {
    //         ch.is_ascii_alphabetic()
    //     });
    //     (word, start, end)
    // }

    /// Checks the previous token kind against the input.
    pub fn checked_lookback(&self, kind: TokenKind) -> bool {
        self.lookback.clone().and_then(|t| if t.kind == kind { Some(true) } else { None }).is_some()
    }

    /// Check if a given keyword follows the keyword rules in the `source`. If not, it is a
    /// `TokenKind::Ident`.
    ///
    /// Rules:
    /// - The `macro`, `fn`, `test`, `function`, `constant`, `event`, `jumptable`,
    ///   `jumptable__packed`, and `table` keywords must be preceded by a `#define` keyword.
    /// - The `takes` keyword must be preceded by an assignment operator: `=`.
    /// - The `nonpayable`, `payable`, `view`, and `pure` keywords must be preceeded by one of these
    ///   keywords or a close paren.
    /// - The `returns` keyword must be succeeded by an open parenthesis and must *not* be succeeded
    ///   by a colon or preceded by the keyword `function`
    pub fn check_keyword_rules(&mut self, found_kind: &Option<TokenKind>) -> bool {
        match found_kind {
            Some(TokenKind::Macro) |
            Some(TokenKind::Fn) |
            Some(TokenKind::Test) |
            Some(TokenKind::Function) |
            Some(TokenKind::Constant) |
            Some(TokenKind::Error) |
            Some(TokenKind::Event) |
            Some(TokenKind::JumpTable) |
            Some(TokenKind::JumpTablePacked) |
            Some(TokenKind::CodeTable) => self.checked_lookback(TokenKind::Define),
            Some(TokenKind::NonPayable) |
            Some(TokenKind::Payable) |
            Some(TokenKind::View) |
            Some(TokenKind::Pure) => {
                let keys = [
                    TokenKind::NonPayable,
                    TokenKind::Payable,
                    TokenKind::View,
                    TokenKind::Pure,
                    TokenKind::CloseParen,
                ];
                for key in keys {
                    if self.checked_lookback(key) {
                        return true
                    }
                }
                false
            }
            Some(TokenKind::Takes) => self.checked_lookback(TokenKind::Assign),
            Some(TokenKind::Returns) => {
                let cur_span_end = self.position;
                self.eat_whitespace();
                // Allow for loose and tight syntax (e.g. `returns   (0)`, `returns(0)`, ...)
                self.peek().unwrap_or(')') == '(' && 
                    !self.checked_lookback(TokenKind::Function)
            }
            _ => true,
        }
    }

}

impl<'a> Iterator for LexerNew<'a> {
    type Item = TokenResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            None
        } else {
            Some(self.next_token())
        }
    }
}