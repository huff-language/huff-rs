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
      parameters: vec![],
      statements: vec![],
      takes: 0,
      returns: 0,
      span: AstSpan(vec![Span::new(0..source.len(), None)]),
    }
  ],
  invocations: vec![],
  imports: vec![],
  constants: vec![],
  functions: vec![],
  events: vec![],
  tables: vec![],
};
assert_eq!(unwrapped_contract, expected_contract);
```