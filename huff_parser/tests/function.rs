use huff_lexer::*;
use huff_parser::*;
use huff_utils::{
    ast::{Function, FunctionType},
    prelude::*,
};
use std::collections::HashMap;

#[test]
fn parses_valid_function_definition() {
    let sources = [
        "#define function test(uint256,bool b) view returns(uint256)",
        "#define function test(uint256) pure returns(uint256)",
        "#define function test(uint256) nonpayable returns(uint256)",
        "#define function test(uint256) payable returns(uint256)",
    ];
    let expected_fns = HashMap::from([
        (
            0,
            Function {
                name: "test".to_string(),
                inputs: vec![
                    Argument {
                        name: None,
                        arg_type: Some(String::from("uint256")),
                        indexed: false,
                    },
                    Argument {
                        name: Some(String::from("b")),
                        arg_type: Some(String::from("bool")),
                        indexed: false,
                    },
                ],
                fn_type: FunctionType::View,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                }],
                signature: [84, 204, 215, 119],
            },
        ),
        (
            1,
            Function {
                name: "test".to_string(),
                inputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                }],
                fn_type: FunctionType::Pure,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                }],
                signature: [41, 233, 159, 7],
            },
        ),
        (
            2,
            Function {
                name: "test".to_string(),
                inputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                }],
                fn_type: FunctionType::NonPayable,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                }],
                signature: [41, 233, 159, 7],
            },
        ),
        (
            3,
            Function {
                name: "test".to_string(),
                inputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                }],
                fn_type: FunctionType::Payable,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                }],
                signature: [41, 233, 159, 7],
            },
        ),
        (
            4,
            Function {
                name: "test".to_string(),
                inputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256[], bool[5]")),
                    indexed: false,
                }],
                fn_type: FunctionType::Payable,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                }],
                signature: [5, 191, 166, 243],
            },
        ),
    ]);

    for (index, source) in sources.into_iter().enumerate() {
        let lexer = Lexer::new(source);
        let tokens = lexer.into_iter().collect::<Vec<Token>>();
        let mut parser = Parser::new(tokens);
        let _ = parser.match_kind(TokenKind::Define);
        let function = parser.parse_function().unwrap();

        // Ensure that the parser constructed the `Function` node correctly.
        assert_eq!(function, *expected_fns.get(&index).unwrap());
    }
}

#[test]
#[should_panic]
fn cannot_parse_invalid_function_definition() {
    let source = "#define function test(uint256) returns(uint256)";
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    parser.parse().unwrap();
}
