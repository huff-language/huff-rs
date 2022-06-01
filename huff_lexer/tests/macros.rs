use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn parses_macros() {
    let source = "#define macro HELLO_WORLD()";
    let lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // TODO:
}
