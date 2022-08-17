## Huff Parser

A parser for the Huff Language.

The Huff Parser accepts a vector of Tokens during instantiation.

Once instantiated, the parser will construct an AST from the Token Vector when the `parse`
method is called.

It also exposes a number of practical methods for accessing information about the source code
throughout lexing.

#### Usage

The following example steps through the lexing of a simple, single-line source code macro
definition.

```rust
use huff_utils::prelude::*;
use huff_lexer::{Lexer};
use huff_parser::{Parser};
use std::sync::{Arc, Mutex};

// Create a Lexer from the source code
let source = "#define macro HELLO_WORLD() = takes(0) returns(0) {}";
let flattened_source = FullFileSource { source, file: None, spans: vec![] };
let mut lexer = Lexer::new(flattened_source);

// Grab the tokens from the lexer
let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();

// Parser incantation
let mut parser = Parser::new(tokens, None);

// Parse into an AST
let unwrapped_contract = parser.parse().unwrap();
assert_eq!(parser.current_token.kind, TokenKind::Eof);

// Validate the unwrapped contract
let expected_contract = Contract {
  macros: vec![
    MacroDefinition {
      name: "HELLO_WORLD".to_string(),
      decorator: None,
      parameters: vec![],
      statements: vec![],
      takes: 0,
      returns: 0,
      span: AstSpan(vec![Span { start: 0, end: 7, file: None }, Span { start: 8, end: 13, file: None }, Span { start: 14, end: 25, file: None }, Span { start: 25, end: 26, file: None }, Span { start: 26, end: 27, file: None }, Span { start: 28, end: 29, file: None }, Span { start: 30, end: 35, file: None }, Span { start: 35, end: 36, file: None }, Span { start: 36, end: 37, file: None }, Span { start: 37, end: 38, file: None }, Span { start: 39, end: 46, file: None }, Span { start: 46, end: 47, file: None }, Span { start: 47, end: 48, file: None }, Span { start: 48, end: 49, file: None }, Span { start: 50, end: 51, file: None }, Span { start: 51, end: 52, file: None }]),
      outlined: false,
      test: false,
    }
  ],
  invocations: vec![],
  imports: vec![],
  constants: Arc::new(Mutex::new(vec![])),
  errors: vec![],
  functions: vec![],
  events: vec![],
  tables: vec![],
};
assert_eq!(unwrapped_contract.macros, expected_contract.macros);
```