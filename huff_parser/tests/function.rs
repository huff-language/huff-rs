use std::collections::HashMap;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;
use huff_utils::ast::{Function, FunctionType};
#[test]
fn parses_valid_function_definition() {
    let sources = [
        "#define function test(uint256,bool) view returns(uint256)",
        "#define function test(uint256) pure returns(uint256)",
        "#define function test(uint256) nonpayable returns(uint256)",
        "#define function test(uint256) payable returns(uint256)",
    ];
    let expected_fns = HashMap::from([
        (0, Function { name: "test", inputs: vec!["uint256".to_string(), "bool".to_string()], fn_type: FunctionType::View, outputs: vec!["uint256".to_string()] }),
        (1, Function { name: "test", inputs: vec!["uint256".to_string()], fn_type: FunctionType::Pure, outputs: vec!["uint256".to_string()] }),
        (2, Function { name: "test", inputs: vec!["uint256".to_string()], fn_type: FunctionType::NonPayable, outputs: vec!["uint256".to_string()] }),
        (3, Function { name: "test", inputs: vec!["uint256".to_string()], fn_type: FunctionType::Payable, outputs: vec!["uint256".to_string()] }),
    ]);

    for source in sources.into_iter().enumerate() {
        let (index, source) = source;
        let lexer = Lexer::new(source);
        let tokens = lexer.into_iter().map(|x| x.unwrap()).filter(|x| !matches!(x.kind, TokenKind::Whitespace)).collect::<Vec<Token>>();
        let mut parser = Parser::new(tokens);
        parser.match_kind(TokenKind::Define);
        let function = parser.parse_function().unwrap();

        // TODO: Ensure that the parser constructed the `Function` node correctly.
        assert_eq!(function, *expected_fns.get(&index).unwrap());
    }
}

#[test]
#[should_panic]
fn cannot_parse_invalid_function_definition() {
    let source = "#define function test(uint256) returns(uint256)";
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    parser.parse();
}
