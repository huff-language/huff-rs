use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn parse_event() {
    let c = "#define event TestEvent(uint256,uint8)";

    let lexer = Lexer::new(c);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    let _ = parser.parse();
    assert_eq!(parser.current_token.kind, TokenKind::Eof);

    // TODO: Ensure that the parser constructed the `Event` node correctly.
}