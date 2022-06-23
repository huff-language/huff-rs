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
                        span: AstSpan(vec![Span { start: 22, end: 29, file: None }]),
                    },
                    Argument {
                        name: Some(String::from("b")),
                        arg_type: Some(String::from("bool")),
                        indexed: false,
                        span: AstSpan(vec![
                            Span { start: 30, end: 34, file: None },
                            Span { start: 35, end: 36, file: None },
                        ]),
                    },
                ],
                fn_type: FunctionType::View,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                    span: AstSpan(vec![Span { start: 51, end: 58, file: None }]),
                }],
                signature: [84, 204, 215, 119],
                span: AstSpan(vec![
                    Span { start: 0, end: 7, file: None },
                    Span { start: 8, end: 16, file: None },
                    Span { start: 17, end: 21, file: None },
                    Span { start: 21, end: 22, file: None },
                    Span { start: 22, end: 29, file: None },
                    Span { start: 29, end: 30, file: None },
                    Span { start: 30, end: 34, file: None },
                    Span { start: 35, end: 36, file: None },
                    Span { start: 36, end: 37, file: None },
                    Span { start: 38, end: 42, file: None },
                    Span { start: 43, end: 50, file: None },
                    Span { start: 50, end: 51, file: None },
                    Span { start: 51, end: 58, file: None },
                    Span { start: 58, end: 59, file: None },
                ]),
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
                    span: AstSpan(vec![Span { start: 22, end: 29, file: None }]),
                }],
                fn_type: FunctionType::Pure,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                    span: AstSpan(vec![Span { start: 44, end: 51, file: None }]),
                }],
                signature: [41, 233, 159, 7],
                span: AstSpan(vec![
                    Span { start: 0, end: 7, file: None },
                    Span { start: 8, end: 16, file: None },
                    Span { start: 17, end: 21, file: None },
                    Span { start: 21, end: 22, file: None },
                    Span { start: 22, end: 29, file: None },
                    Span { start: 29, end: 30, file: None },
                    Span { start: 31, end: 35, file: None },
                    Span { start: 36, end: 43, file: None },
                    Span { start: 43, end: 44, file: None },
                    Span { start: 44, end: 51, file: None },
                    Span { start: 51, end: 52, file: None },
                ]),
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
                    span: AstSpan(vec![Span { start: 22, end: 29, file: None }]),
                }],
                fn_type: FunctionType::NonPayable,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                    span: AstSpan(vec![Span { start: 50, end: 57, file: None }]),
                }],
                signature: [41, 233, 159, 7],
                span: AstSpan(vec![
                    Span { start: 0, end: 7, file: None },
                    Span { start: 8, end: 16, file: None },
                    Span { start: 17, end: 21, file: None },
                    Span { start: 21, end: 22, file: None },
                    Span { start: 22, end: 29, file: None },
                    Span { start: 29, end: 30, file: None },
                    Span { start: 31, end: 41, file: None },
                    Span { start: 42, end: 49, file: None },
                    Span { start: 49, end: 50, file: None },
                    Span { start: 50, end: 57, file: None },
                    Span { start: 57, end: 58, file: None },
                ]),
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
                    span: AstSpan(vec![Span { start: 22, end: 29, file: None }]),
                }],
                fn_type: FunctionType::Payable,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                    span: AstSpan(vec![Span { start: 47, end: 54, file: None }]),
                }],
                signature: [41, 233, 159, 7],
                span: AstSpan(vec![
                    Span { start: 0, end: 7, file: None },
                    Span { start: 8, end: 16, file: None },
                    Span { start: 17, end: 21, file: None },
                    Span { start: 21, end: 22, file: None },
                    Span { start: 22, end: 29, file: None },
                    Span { start: 29, end: 30, file: None },
                    Span { start: 31, end: 38, file: None },
                    Span { start: 39, end: 46, file: None },
                    Span { start: 46, end: 47, file: None },
                    Span { start: 47, end: 54, file: None },
                    Span { start: 54, end: 55, file: None },
                ]),
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
                    span: AstSpan(vec![]),
                }],
                fn_type: FunctionType::Payable,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                    span: AstSpan(vec![]),
                }],
                signature: [5, 191, 166, 243],
                span: AstSpan(vec![]),
            },
        ),
    ]);

    for (index, source) in sources.into_iter().enumerate() {
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(flattened_source);
        let tokens = lexer
            .into_iter()
            .map(|x| x.unwrap())
            .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
            .collect::<Vec<Token>>();
        let mut parser = Parser::new(tokens, None);
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
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    parser.parse().unwrap();
}
