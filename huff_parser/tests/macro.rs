use proptest::prelude::*;

use huff_parser::*;
use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn empty_macro() {
    let source = "#define macro HELLO_WORLD()";
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    parser.parse();
    assert_eq!(parser.current_token.kind, TokenKind::Whitespace);
}