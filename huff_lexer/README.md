## Huff Lexer

Lexical analyzer for the Huff Language.

The Huff Lexer is instantiable with a string representing the source code.

Once instantiated, the lexer can be used to iterate over the tokens in the source code.
It also exposes a number of practical methods for accessing information about the source code
throughout lexing.

#### Usage

The following example steps through the lexing of a simple, single-line source code macro
definition.

```rust
use huff_utils::{token::*, span::*};
use huff_lexer::{Lexer};

// Instantiate a new lexer
let source = "#define macro HELLO_WORLD()";
let mut lexer = Lexer::new(source);
assert_eq!(lexer.source, source);

// This token should be a Define identifier
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::Define, Span::new(0..7)));
assert_eq!(lexer.span, Span::new(0..7));

// The next token should be the whitespace
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::Whitespace, Span::new(7..8)));
assert_eq!(lexer.span, Span::new(7..8));

// Then we should parse the macro keyword
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::Macro, Span::new(8..13)));
assert_eq!(lexer.span, Span::new(8..13));

// The next token should be another whitespace
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::Whitespace, Span::new(13..14)));
assert_eq!(lexer.span, Span::new(13..14));

// Then we should get the function name
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::Ident("HELLO_WORLD".to_string()), Span::new(14..25)));
assert_eq!(lexer.span, Span::new(14..25));

// Then we should have an open paren
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::OpenParen, Span::new(25..26)));
assert_eq!(lexer.span, Span::new(25..26));

// Lastly, we should have a closing parenthesis
let tok = lexer.next().unwrap().unwrap();
assert_eq!(tok, Token::new(TokenKind::CloseParen, Span::new(26..27)));
assert_eq!(lexer.span, Span::new(26..27));

// We covered the whole source
assert_eq!(lexer.span.end, source.len());
assert!(lexer.eof);
```
