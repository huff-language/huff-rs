use huff_utils::prelude::*;
use regex::Regex;
use std::{
    iter::Peekable,
    str::Chars,
};

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
            match ch {
                '/' => {
                    if let Some(ch2) = self.peek() {
                        match ch2 {
                            '/' => {
                                // Consume until newline
                                // let c = self.eat_while(None, |c| *c != '\n');
                            }
                        }
                    }
                }
                // # keywords
                '#' => {

                }
                // Alphabetical characters
                ch if ch.is_alphabetic() || ch.eq(&'_') => {

                }
                // If it's the start of a hex literal
                ch if ch == '0' && self.peek().unwrap() == 'x' => {

                }
                '=' => TokenKind::Assign,
                '(' => {

                }
                ')' => {

                }
                '[' => TokenKind::OpenBracket,
                ']' => TokenKind::CloseBracket,
                '{' => {
                    if self.context == Context::MacroDefinition {
                        self.context = Context::MacroBody;
                    }
                    TokenKind::OpenBrace
                }
                '}' => {
                    if matches!(self.context, Context::MacroBody | Context::CodeTableBody) {
                        self.context = Context::Global;
                    }
                    TokenKind::CloseBrace
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
                '"' => {
                    // loop {
                    //     match self.peek() {
                    //         Some('"') => {
                    //             self.consume();
                    //             let str = self.slice();
                    //             break TokenKind::Str((str[1..str.len() - 1]).to_string())
                    //         }
                    //         Some('\\') if matches!(self.nth_peek(1), Some('\\') | Some('"')) => {
                    //             self.consume();
                    //         }
                    //         Some(_) => {}
                    //         None => {
                    //             self.eof = true;
                    //             tracing::error!(target: "lexer", "UNEXPECTED EOF SPAN");
                    //             return Some(Err(LexicalError::new(
                    //                 LexicalErrorKind::UnexpectedEof,
                    //                 self.current_span().clone(),
                    //             )))
                    //         }
                    //     }
                    //     self.consume();
                    // }
                }
                '\'' => {

                }
                ch => {
                    tracing::error!(target: "lexer", "UNSUPPORTED TOKEN '{}'", ch);
                    return Err(LexicalError::new(
                        LexicalErrorKind::InvalidCharacter(ch),
                        Span { start: self.position as usize, end: self.position as usize, file: None },
                    ))
                }
            }
            // TODO: change this to have a starting position 
            // Ok(Token { kind: token_kind, span: Span { start: self.position as usize, end: self.position as usize, file: None } })
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

    /// Skips white space. They are not significant in the source language
    fn eat_whitespace(&mut self) {
        self.eat_while(None, |ch| ch.is_whitespace());
    }

    fn eat_string_literal(&mut self) -> SpannedToken {
        let (str_literal, start_span, end_span) = self.eat_while(None, |ch| ch != '"');
        let str_literal_token = Token::Str(str_literal);
        self.next_char(); // Advance past the closing quote
        str_literal_token.into_span(start_span, end_span)
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