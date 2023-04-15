## Huff Lexer

Lexical analyzer for the Huff Language.

The Huff Lexer is instantiable with a `FullFileSource`. `FullFileSource` is composed of
- the file source code
- a `FileSource`
- `spans` == (`Vec<(FileSource, Span)>`).

Once instantiated, the lexer can be used to iterate over the tokens in the source code.
It also exposes a number of practical methods for accessing information about the source code
throughout lexing.

#### Usage

The following example steps through the lexing of a simple, single-line source code macro
definition.

```rust
use huff_utils::prelude::*;
use huff_lexer::{Lexer};
use std::ops::Deref;

// Instantiate a new lexer
let source = "#define macro HELLO_WORLD()";
let flattened_source = FullFileSource { source, file: None, spans: vec![] };
let mut lexer = Lexer::new(flattened_source.sour);

// This token should be a Define identifier
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::Define, Span::new(0..6, None)));

// The next token should be the whitespace
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::Whitespace, Span::new(7..7, None)));

// Then we should parse the macro keyword
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::Macro, Span::new(8..12, None)));

// The next token should be another whitespace
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::Whitespace, Span::new(13..13, None)));

// Then we should get the function name
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::Ident("HELLO_WORLD".to_string()), Span::new(14..24, None)));

// Then we should have an open paren
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::OpenParen, Span::new(25..25, None)));

// Lastly, we should have a closing parenthesis
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::CloseParen, Span::new(26..26, None)));

lexer.next();
// We covered the whole source
assert!(lexer.eof);
```
