use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn parses_function_definition() {
    let sources = [
        "#define function test(uint256) view returns(uint256)",
        "#define function test(uint256) pure returns(uint256)",
        "#define function test(uint256) nonpayable returns(uint256)",
        "#define function test(uint256) payable returns(uint256)",
        "#define function test(uint256) returns(uint256)",
    ];

    for source in sources {
        let lexer = Lexer::new(source);
        let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
        let mut parser = Parser::new(tokens);
        parser.parse();
        assert_eq!(parser.current_token.kind, TokenKind::Eof);

        // TODO: Ensure that the parser constructed the `Function` node correctly.
    }
}
