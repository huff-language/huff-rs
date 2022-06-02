use proptest::prelude::*;

use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn empty_macro() {
    let source = "#define macro HELLO_WORLD()";
    let lexer = Lexer::new(source);
    let tokens = lexer.iter().collect::<Vec<Token<'a>>>();
    assert_eq!(lexer.source, source);
    assert_eq!(lexer.span, Span::default());
    assert!(!lexer.eof);
}