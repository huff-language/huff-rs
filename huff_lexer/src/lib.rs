#![doc = include_str!("../README.md")]
#![allow(dead_code)]
#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]

use huff_utils::{bytes_util::*, error::*, evm::*, span::*, token::*, types::*};
use regex::Regex;
use std::{iter::Peekable, str::Chars};

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
    /// ABI context
    Abi,
    /// Lexing args of functions inputs/outputs and events
    AbiArgs,
    /// constant context
    Constant,
}

/// ## Lexer
///
/// The lexer encapsulated in a struct.
pub struct Lexer<'a> {
    /// The source code as peekable chars.
    /// SHOULD NOT BE MODIFIED EVER!
    pub reference_chars: Peekable<Chars<'a>>,
    /// The source code as peekable chars.
    pub chars: Peekable<Chars<'a>>,
    /// The raw source code.
    pub source: &'a str,
    /// The current lexing span.
    pub span: Span,
    /// The previous lexed Token.
    /// Cannot be a whitespace.
    pub lookback: Option<Token>,
    /// If the lexer has reached the end of file.
    pub eof: bool,
    /// EOF Token has been returned.
    pub eof_returned: bool,
    /// Current context.
    pub context: Context,
}

impl<'a> Lexer<'a> {
    /// Public associated function that instantiates a new lexer.
    pub fn new(source: &'a str) -> Self {
        Self {
            reference_chars: source.chars().peekable(),
            chars: source.chars().peekable(),
            source,
            span: Span::default(),
            lookback: None,
            eof: false,
            eof_returned: false,
            context: Context::Global,
        }
    }

    // TODO: This does not account for commented out imports for example:
    // `// #include "./Utils.huff"`
    /// Lex all imports
    pub fn lex_imports(source: &str) -> Vec<String> {
        let mut imports = vec![];
        let mut peekable_source = source.chars().peekable();
        let mut include_chars_iterator = "#include".chars().peekable();
        while peekable_source.peek().is_some() {
            while let Some(nc) = peekable_source.next() {
                if include_chars_iterator.peek().is_none() {
                    // Reset the include chars iterator
                    include_chars_iterator = "#include".chars().peekable();

                    // Skip over whitespace
                    while peekable_source.peek().is_some() {
                        if !peekable_source.peek().unwrap().is_whitespace() {
                            break
                        } else {
                            peekable_source.next();
                        }
                    }

                    // Then we should have an import path between quotes
                    match peekable_source.peek() {
                        Some(char) => match char {
                            '"' | '\'' => {
                                peekable_source.next();
                                let mut import = String::new();
                                while peekable_source.peek().is_some() {
                                    match peekable_source.next().unwrap() {
                                        '"' | '\'' => {
                                            imports.push(import);
                                            break
                                        }
                                        c => import.push(c),
                                    }
                                }
                            }
                            _ => { /* Ignore non-include tokens */ }
                        },
                        None => { /* EOF */ }
                    }
                } else if nc != include_chars_iterator.next().unwrap() {
                    include_chars_iterator = "#include".chars().peekable();
                    break
                }
            }
        }
        imports
    }

    /// Public associated function that returns the current lexing span.
    pub fn current_span(&self) -> Span {
        if self.eof {
            Span::EOF
        } else {
            self.span
        }
    }

    /// Get the length of the previous lexing span.
    pub fn lookback_len(&self) -> usize {
        if let Some(lookback) = &self.lookback {
            return lookback.span.end - lookback.span.start
        }
        0
    }

    /// Checks the previous token kind against the input.
    pub fn checked_lookback(&self, kind: TokenKind) -> bool {
        self.lookback.clone().and_then(|t| if t.kind == kind { Some(true) } else { None }).is_some()
    }

    /// Try to peek at the next character from the source
    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// Dynamically peeks characters based on the filter
    pub fn dyn_peek(&mut self, f: impl Fn(&char) -> bool + Copy) -> String {
        let mut chars: Vec<char> = Vec::new();
        let mut current_pos = self.span.start;
        while self.nth_peek(current_pos).map(|x| f(&x)).unwrap_or(false) {
            chars.push(self.nth_peek(current_pos).unwrap());
            current_pos += 1;
        }
        chars.iter().collect()
    }

    /// Try to peek at the nth character from the source
    pub fn nth_peek(&mut self, n: usize) -> Option<char> {
        self.reference_chars.clone().nth(n)
    }

    /// Try to peek at next n characters from the source
    pub fn peek_n_chars(&mut self, n: usize) -> String {
        let mut newspan: Span = self.span;
        newspan.end += n;
        // Break with an empty string if the bounds are exceeded
        if newspan.end > self.source.len() {
            return String::default()
        }
        self.source[newspan.range().unwrap()].to_string()
    }

    /// Peek n chars from a given start point in the source
    pub fn peek_n_chars_from(&mut self, n: usize, from: usize) -> String {
        self.source[Span::new(from..(from + n)).range().unwrap()].to_string()
    }

    /// Gets the current slice of the source code covered by span
    pub fn slice(&self) -> &'a str {
        &self.source[self.span.range().unwrap()]
    }

    /// Consumes the characters
    pub fn consume(&mut self) -> Option<char> {
        self.chars.next().map(|x| {
            self.span.end += 1;
            x
        })
    }

    /// Consumes n characters
    pub fn nconsume(&mut self, count: usize) {
        for _ in 0..count {
            let _ = self.consume();
        }
    }

    /// Consume characters until a sequence matches
    pub fn seq_consume(&mut self, word: &str) {
        let mut current_pos = self.span.start;
        while self.peek() != None {
            let peeked = self.peek_n_chars_from(word.len(), current_pos);
            if word == peeked {
                break
            }
            self.consume();
            current_pos += 1;
        }
    }

    /// Dynamically consumes characters based on filters
    pub fn dyn_consume(&mut self, f: impl Fn(&char) -> bool + Copy) {
        while self.peek().map(|x| f(&x)).unwrap_or(false) {
            self.consume();
        }
    }

    /// Resets the Lexer's span
    ///
    /// Only sets the previous span if the current token is not a whitespace.
    pub fn reset(&mut self) {
        self.span.start = self.span.end;
    }

    /// Check if a given keyword follows the keyword rules in the `source`. If not, it is a
    /// `TokenKind::Ident`.
    ///
    /// Rules:
    /// - The `macro`, `function`, `constant`, `event` keywords must be preceded by a `#define`
    ///   keyword.
    /// - The `takes` keyword must be preceded by an assignment operator: `=`.
    /// - The `nonpayable`, `payable`, `view`, and `pure` keywords must be preceeded by one of these
    ///   keywords or a close paren.
    /// - The `returns` keyword must be succeeded by an open parenthesis and must *not* be succeeded
    ///   by a colon or preceded by the keyword `function`
    pub fn check_keyword_rules(&mut self, found_kind: &Option<TokenKind>) -> bool {
        match found_kind {
            Some(TokenKind::Macro) |
            Some(TokenKind::Function) |
            Some(TokenKind::Constant) |
            Some(TokenKind::Event) => self.checked_lookback(TokenKind::Define),
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
                // Allow for loose and tight syntax (e.g. `returns (0)` & `returns(0)`)
                self.peek_n_chars_from(2, self.span.end).trim().starts_with('(') &&
                    !self.checked_lookback(TokenKind::Function) &&
                    self.peek_n_chars_from(1, self.span.end) != ":"
            }
            _ => true,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexicalError<'a>>;

    /// Iterates over the source code
    fn next(&mut self) -> Option<Self::Item> {
        self.reset();
        if let Some(ch) = self.consume() {
            let kind = match ch {
                // Comments
                '/' => {
                    if let Some(ch2) = self.peek() {
                        match ch2 {
                            '/' => {
                                self.consume();
                                // Consume until newline
                                self.dyn_consume(|c| *c != '\n');
                                TokenKind::Comment(self.slice().to_string())
                            }
                            '*' => {
                                self.consume();
                                // Consume until next '*/' occurance
                                self.seq_consume("*/");
                                TokenKind::Comment(self.slice().to_string())
                            }
                            _ => TokenKind::Div,
                        }
                    } else {
                        TokenKind::Div
                    }
                }
                // # keywords
                '#' => {
                    let mut found_kind: Option<TokenKind> = None;

                    let keys = [TokenKind::Define, TokenKind::Include];
                    for kind in &keys {
                        let key = kind.to_string();
                        let token_length = key.len() - 1;
                        let peeked = self.peek_n_chars(token_length);

                        if *key == peeked {
                            self.nconsume(token_length);
                            found_kind = Some(kind.clone());
                            break
                        }
                    }

                    if let Some(kind) = found_kind {
                        kind
                    } else {
                        // Otherwise we don't support # prefixed indentifiers
                        return Some(Err(LexicalError::new(
                            LexicalErrorKind::InvalidCharacter('#'),
                            self.current_span(),
                        )))
                    }
                }
                // Alphabetical characters
                ch if ch.is_alphabetic() => {
                    let mut found_kind: Option<TokenKind> = None;

                    let keys = [
                        TokenKind::Macro,
                        TokenKind::Function,
                        TokenKind::Constant,
                        TokenKind::Takes,
                        TokenKind::Returns,
                        TokenKind::Event,
                        TokenKind::NonPayable,
                        TokenKind::Payable,
                        TokenKind::Indexed,
                        TokenKind::View,
                        TokenKind::Pure,
                    ];
                    for kind in &keys {
                        if self.context == Context::MacroBody {
                            break
                        }
                        let key = kind.to_string();
                        let token_length = key.len() - 1;
                        let peeked = self.peek_n_chars(token_length);

                        if *key == peeked {
                            self.nconsume(token_length);
                            found_kind = Some(kind.clone());
                            break
                        }
                    }

                    // Check to see if the found kind is, in fact, a keyword and not the name of
                    // a function. If it is, set `found_kind` to `None` so that it is set to a
                    // `TokenKind::Ident` in the following control flow.
                    if !self.check_keyword_rules(&found_kind) {
                        found_kind = None;
                    }

                    if let Some(tokind) = &found_kind {
                        match tokind {
                            TokenKind::Macro => self.context = Context::MacroDefinition,
                            TokenKind::Function | TokenKind::Event => self.context = Context::Abi,
                            TokenKind::Constant => self.context = Context::Constant,
                            _ => (),
                        }
                    }

                    // Check for macro keyword
                    let fsp = "FREE_STORAGE_POINTER";
                    let token_length = fsp.len() - 1;
                    let peeked = self.peek_n_chars(token_length);
                    if fsp == peeked {
                        self.nconsume(token_length);
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

                    let potential_label: String =
                        self.dyn_peek(|c| c.is_alphanumeric() || c == &'_' || c == &':');
                    if let true = potential_label.ends_with(':') {
                        self.dyn_consume(|c| c.is_alphanumeric() || c == &'_' || c == &':');
                        let label = self.slice();
                        if let Some(l) = label.get(0..label.len() - 1) {
                            found_kind = Some(TokenKind::Label(l.to_string()));
                        } else {
                            tracing::error!("[huff_lexer] Fatal Label Colon Truncation!");
                        }
                    }

                    let pot_op = self.dyn_peek(|c| c.is_alphanumeric());
                    // goes over all opcodes
                    for opcode in OPCODES {
                        if self.context != Context::MacroBody {
                            break
                        }
                        if opcode == pot_op {
                            self.dyn_consume(|c| c.is_alphanumeric());
                            if let Some(o) = OPCODES_MAP.get(opcode) {
                                found_kind = Some(TokenKind::Opcode(o.to_owned()));
                            } else {
                                tracing::error!("[huff_lexer] Fatal Opcode Mapping!");
                            }
                            break
                        }
                    }

                    // Last case ; we are in ABI context and
                    // we are parsing an EVM type
                    if self.context == Context::AbiArgs {
                        let curr_char = self.peek()?;
                        if !['(', ')'].contains(&curr_char) {
                            self.dyn_consume(|c| c.is_alphanumeric() || *c == '[' || *c == ']');
                            // got a type at this point, we have to know which
                            let raw_type: &str = self.slice();
                            // check for arrays first
                            if EVM_TYPE_ARRAY_REGEX.is_match(raw_type) {
                                // split to get array size and type
                                // TODO: support multi-dimensional arrays
                                let words: Vec<String> = Regex::new(r"\[")
                                    .unwrap()
                                    .split(raw_type)
                                    .map(|x| x.replace(']', ""))
                                    .collect();

                                let mut size_vec: Vec<usize> = Vec::new();
                                // go over all array sizes
                                let sizes = words.get(1..words.len()).unwrap();
                                for size in sizes.iter() {
                                    match size.is_empty() {
                                        true => size_vec.push(0),
                                        false => {
                                            let arr_size: usize = size
                                                .parse::<usize>()
                                                .map_err(|_| {
                                                    let err = LexicalError {
                                                        kind: LexicalErrorKind::InvalidArraySize(
                                                            &words[1],
                                                        ),
                                                        span: self.span,
                                                    };
                                                    tracing::error!("{}", format!("{:?}", err));
                                                    err
                                                })
                                                .unwrap();
                                            size_vec.push(arr_size);
                                        }
                                    }
                                }
                                let primitive = PrimitiveEVMType::try_from(words[0].clone());
                                if let Ok(primitive) = primitive {
                                    found_kind = Some(TokenKind::ArrayType(primitive, size_vec));
                                } else {
                                    let err = LexicalError {
                                        kind: LexicalErrorKind::InvalidPrimitiveType(&words[0]),
                                        span: self.span,
                                    };
                                    tracing::error!("{}", format!("{:?}", err));
                                }
                            } else {
                                // We don't want to consider any argument names or the "indexed"
                                // keyword here.
                                let primitive = PrimitiveEVMType::try_from(raw_type.to_string());
                                if let Ok(primitive) = primitive {
                                    found_kind = Some(TokenKind::PrimitiveType(primitive));
                                }
                            }
                        }
                    }

                    if let Some(kind) = &found_kind {
                        kind.clone()
                    } else {
                        self.dyn_consume(|c| c.is_alphanumeric() || c.eq(&'_'));
                        TokenKind::Ident(self.slice().to_string())
                    }
                }
                // If it's the start of a hex literal
                ch if ch == '0' && self.peek().unwrap() == 'x' => {
                    self.consume(); // Consume the 'x' after '0' (separated from the `dyn_consume` so we don't have
                                    // to match `x` in the actual hex)
                    self.dyn_consume(|c| {
                        c.is_numeric() ||
                            // Match a-f & A-F
                            matches!(c, '\u{0041}'..='\u{0046}' | '\u{0061}'..='\u{0066}')
                    });
                    self.span.start += 2; // Ignore the "0x"
                    TokenKind::Literal(str_to_bytes32(self.slice()))
                }
                '=' => TokenKind::Assign,
                '(' => {
                    if self.context == Context::Abi {
                        self.context = Context::AbiArgs;
                    }
                    TokenKind::OpenParen
                }
                ')' => {
                    if self.context == Context::AbiArgs {
                        self.context = Context::Abi;
                    }
                    TokenKind::CloseParen
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
                    if self.context == Context::MacroBody {
                        self.context = Context::Global;
                    }
                    TokenKind::CloseBrace
                }
                '+' => TokenKind::Add,
                '-' => TokenKind::Sub,
                '*' => TokenKind::Mul,
                '<' => TokenKind::LeftAngle,
                '>' => TokenKind::RightAngle,
                // NOTE: TokenKind::Div is lexed further up since it overlaps with comment
                ':' => TokenKind::Colon,
                // identifiers
                ',' => TokenKind::Comma,
                '0'..='9' => {
                    self.dyn_consume(char::is_ascii_digit);
                    TokenKind::Num(self.slice().parse().unwrap())
                }
                // Lexes Spaces and Newlines as Whitespace
                ch if ch.is_ascii_whitespace() => {
                    self.dyn_consume(char::is_ascii_whitespace);
                    TokenKind::Whitespace
                }
                // String literals
                '"' => loop {
                    match self.peek() {
                        Some('"') => {
                            self.consume();
                            let str = self.slice();
                            break TokenKind::Str((&str[1..str.len() - 1]).to_string())
                        }
                        Some('\\') if matches!(self.nth_peek(1), Some('\\') | Some('"')) => {
                            self.consume();
                        }
                        Some(_) => {}
                        None => {
                            self.eof = true;
                            return Some(Err(LexicalError::new(
                                LexicalErrorKind::UnexpectedEof,
                                self.span,
                            )))
                        }
                    }
                    self.consume();
                },
                // Allow string literals to be wrapped by single quotes
                '\'' => loop {
                    match self.peek() {
                        Some('\'') => {
                            self.consume();
                            let str = self.slice();
                            break TokenKind::Str((&str[1..str.len() - 1]).to_string())
                        }
                        Some('\\') if matches!(self.nth_peek(1), Some('\\') | Some('\'')) => {
                            self.consume();
                        }
                        Some(_) => {}
                        None => {
                            self.eof = true;
                            return Some(Err(LexicalError::new(
                                LexicalErrorKind::UnexpectedEof,
                                self.span,
                            )))
                        }
                    }
                    self.consume();
                },
                // At this point, the source code has an invalid or unsupported token
                ch => {
                    return Some(Err(LexicalError::new(
                        LexicalErrorKind::InvalidCharacter(ch),
                        self.span,
                    )))
                }
            };

            if self.peek().is_none() {
                self.eof = true;
            }

            let token = Token { kind, span: self.span };
            if token.kind != TokenKind::Whitespace {
                self.lookback = Some(token.clone());
            }

            return Some(Ok(token))
        }

        // Mark EOF
        self.eof = true;

        // If we haven't returned an eof token, return one
        if !self.eof_returned {
            self.eof_returned = true;
            let token = Token { kind: TokenKind::Eof, span: self.span };
            if token.kind != TokenKind::Whitespace {
                self.lookback = Some(token.clone());
            }
            return Some(Ok(token))
        }

        None
    }
}
