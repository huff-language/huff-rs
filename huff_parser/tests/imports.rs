use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn parses_import() {
    let c = " /* .,*./. */  #include \"./utils/ERC1155.huff\"";

    let lexer = Lexer::new(c);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    let contract = parser.parse().unwrap();
    assert_eq!(parser.current_token.kind, TokenKind::Eof);

    let import_path = contract.imports[0];
    assert_eq!(import_path, "./utils/ERC1155.huff");
}
