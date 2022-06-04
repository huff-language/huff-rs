use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn parses_function_definition() {
    let source = "#define function test(uint256) view returns(uint256)";
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    parser.parse();
    assert_eq!(parser.current_token.kind, TokenKind::Eof);

    // TODO: Ensure that the parser constructed the `Function` node correctly.
}