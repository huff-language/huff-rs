## Huff Parser

A parser for the Huff Language.

The Huff Parser accepts a vector of Tokens during instantiation.

Once instantiated, the par&ser will construct an AST from the Token Vector when the `parse`
method is called.

It also exposes a number of practical methods for accessing information about the source code
throughout lexing.

#### Usage

The following example steps through the lexing of a simple, single-line source code macro
definition.

```rust
use huff_utils::{token::*, span::*};
use huff_lexer::{Lexer};
use huff_parser::{Parser};

// Mock source code as a string
let source = "#define macro HELLO_WORLD() = takes(0) returns(0) {}";

// Create a lexer from the source code
let lexer = Lexer::new(source);

// Grab the tokens from the lexer
let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();

// Parser incantation
let mut parser = Parser::new(tokens, None);

// Parse into an AST
parser.parse();
assert_eq!(parser.current_token.kind, TokenKind::Eof);
```