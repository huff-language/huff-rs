use proptest::prelude::*;

use huff_parser::*;
use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn empty_macro() {
    let source = "#define macro HELLO_WORLD()";
    let lexer = Lexer::new(source);
    let tokens = lexer.iter().collect::<Vec<Token>>();
    let parser = Parser::new(tokens);
    let result = parser.parse().unwrap();
}