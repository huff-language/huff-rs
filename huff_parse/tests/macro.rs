use huff_parse::{*, ParserError};
use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn empty_macro() {
    let source = "#define macro HELLOWORLD() = takes(045) returns(0) {}";
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    let macro_def = parser.parse_macro();
    println!("{:?}", macro_def);
}