use huff_lexer::*;
use huff_parser::*;
use huff_utils::{
    ast::{FunctionDefinition, FunctionType},
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
            FunctionDefinition {
                name: "test".to_string(),
                inputs: vec![
                    Argument {
                        name: None,
                        arg_type: Some(String::from("uint256")),
                        indexed: false,
                        arg_location: None,
                        span: AstSpan(vec![Span { start: 22, end: 28, file: None }]),
                    },
                    Argument {
                        name: Some(String::from("b")),
                        arg_type: Some(String::from("bool")),
                        indexed: false,
                        arg_location: None,
                        span: AstSpan(vec![
                            Span { start: 30, end: 33, file: None },
                            Span { start: 35, end: 35, file: None },
                        ]),
                    },
                ],
                fn_type: FunctionType::View,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                    arg_location: None,
                    span: AstSpan(vec![Span { start: 51, end: 57, file: None }]),
                }],
                signature: [84, 204, 215, 119],
                span: AstSpan(vec![
                    Span { start: 0, end: 6, file: None },
                    Span { start: 8, end: 15, file: None },
                    Span { start: 17, end: 20, file: None },
                    Span { start: 21, end: 21, file: None },
                    Span { start: 22, end: 28, file: None },
                    Span { start: 29, end: 29, file: None },
                    Span { start: 30, end: 33, file: None },
                    Span { start: 35, end: 35, file: None },
                    Span { start: 36, end: 36, file: None },
                    Span { start: 38, end: 41, file: None },
                    Span { start: 43, end: 49, file: None },
                    Span { start: 50, end: 50, file: None },
                    Span { start: 51, end: 57, file: None },
                    Span { start: 58, end: 58, file: None },
                ]),
            },
        ),
        (
            1,
            FunctionDefinition {
                name: "test".to_string(),
                inputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                    arg_location: None,
                    span: AstSpan(vec![Span { start: 22, end: 28, file: None }]),
                }],
                fn_type: FunctionType::Pure,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                    arg_location: None,
                    span: AstSpan(vec![Span { start: 44, end: 50, file: None }]),
                }],
                signature: [41, 233, 159, 7],
                span: AstSpan(vec![
                    Span { start: 0, end: 6, file: None },
                    Span { start: 8, end: 15, file: None },
                    Span { start: 17, end: 20, file: None },
                    Span { start: 21, end: 21, file: None },
                    Span { start: 22, end: 28, file: None },
                    Span { start: 29, end: 29, file: None },
                    Span { start: 31, end: 34, file: None },
                    Span { start: 36, end: 42, file: None },
                    Span { start: 43, end: 43, file: None },
                    Span { start: 44, end: 50, file: None },
                    Span { start: 51, end: 51, file: None },
                ]),
            },
        ),
        (
            2,
            FunctionDefinition {
                name: "test".to_string(),
                inputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                    arg_location: None,
                    span: AstSpan(vec![Span { start: 22, end: 28, file: None }]),
                }],
                fn_type: FunctionType::NonPayable,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                    arg_location: None,
                    span: AstSpan(vec![Span { start: 50, end: 56, file: None }]),
                }],
                signature: [41, 233, 159, 7],
                span: AstSpan(vec![
                    Span { start: 0, end: 6, file: None },
                    Span { start: 8, end: 15, file: None },
                    Span { start: 17, end: 20, file: None },
                    Span { start: 21, end: 21, file: None },
                    Span { start: 22, end: 28, file: None },
                    Span { start: 29, end: 29, file: None },
                    Span { start: 31, end: 40, file: None },
                    Span { start: 42, end: 48, file: None },
                    Span { start: 49, end: 49, file: None },
                    Span { start: 50, end: 56, file: None },
                    Span { start: 57, end: 57, file: None },
                ]),
            },
        ),
        (
            3,
            FunctionDefinition {
                name: "test".to_string(),
                inputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                    arg_location: None,
                    span: AstSpan(vec![Span { start: 22, end: 28, file: None }]),
                }],
                fn_type: FunctionType::Payable,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                    arg_location: None,
                    span: AstSpan(vec![Span { start: 47, end: 53, file: None }]),
                }],
                signature: [41, 233, 159, 7],
                span: AstSpan(vec![
                    Span { start: 0, end: 6, file: None },
                    Span { start: 8, end: 15, file: None },
                    Span { start: 17, end: 20, file: None },
                    Span { start: 21, end: 21, file: None },
                    Span { start: 22, end: 28, file: None },
                    Span { start: 29, end: 29, file: None },
                    Span { start: 31, end: 37, file: None },
                    Span { start: 39, end: 45, file: None },
                    Span { start: 46, end: 46, file: None },
                    Span { start: 47, end: 53, file: None },
                    Span { start: 54, end: 54, file: None },
                ]),
            },
        ),
        (
            4,
            FunctionDefinition {
                name: "test".to_string(),
                inputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256[], bool[5]")),
                    indexed: false,
                    arg_location: None,
                    span: AstSpan(vec![]),
                }],
                fn_type: FunctionType::Payable,
                outputs: vec![Argument {
                    name: None,
                    arg_type: Some(String::from("uint256")),
                    indexed: false,
                    arg_location: None,
                    span: AstSpan(vec![]),
                }],
                signature: [5, 191, 166, 243],
                span: AstSpan(vec![]),
            },
        ),
    ]);

    for (index, source) in sources.into_iter().enumerate() {
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(flattened_source.source);
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
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    parser.parse().unwrap();
}

#[test]
#[should_panic]
fn test_functions_with_keyword_arg_names_errors() {
    // The function parameter's name is a reserved keyword; this should throw an error
    let source: &str = "#define function myFunc(uint256 uint256) pure returns(uint256)";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    parser.parse().unwrap();
}

#[test]
fn test_functions_with_argument_locations() {
    let source: &str = "#define function myFunc(string calldata test, uint256[] storage) pure returns(bytes memory)";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    parser.parse().unwrap();
}

#[test]
fn test_can_prefix_function_arg_names_with_reserved_keywords() {
    let source: &str = "#define function supportsInterface(bytes4 interfaceId) view returns (bool)";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let expected_tokens: Vec<Token> = vec![
        Token { kind: TokenKind::Define, span: Span { start: 0, end: 6, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 7, end: 7, file: None } },
        Token { kind: TokenKind::Function, span: Span { start: 8, end: 15, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 16, end: 16, file: None } },
        Token {
            kind: TokenKind::Ident("supportsInterface".to_string()),
            span: Span { start: 17, end: 33, file: None },
        },
        Token { kind: TokenKind::OpenParen, span: Span { start: 34, end: 34, file: None } },
        Token {
            kind: TokenKind::PrimitiveType(PrimitiveEVMType::Bytes(4)),
            span: Span { start: 35, end: 40, file: None },
        },
        Token { kind: TokenKind::Whitespace, span: Span { start: 41, end: 41, file: None } },
        Token {
            kind: TokenKind::Ident("interfaceId".to_string()),
            span: Span { start: 42, end: 52, file: None },
        },
        Token { kind: TokenKind::CloseParen, span: Span { start: 53, end: 53, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 54, end: 54, file: None } },
        Token { kind: TokenKind::View, span: Span { start: 55, end: 58, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 59, end: 59, file: None } },
        Token { kind: TokenKind::Returns, span: Span { start: 60, end: 66, file: None } },
        Token { kind: TokenKind::OpenParen, span: Span { start: 68, end: 68, file: None } },
        Token {
            kind: TokenKind::PrimitiveType(PrimitiveEVMType::Bool),
            span: Span { start: 69, end: 72, file: None },
        },
        Token { kind: TokenKind::CloseParen, span: Span { start: 73, end: 73, file: None } },
        Token { kind: TokenKind::Eof, span: Span { start: 73, end: 73, file: None } },
    ];
    assert_eq!(expected_tokens, tokens);
    let mut parser = Parser::new(tokens, None);
    parser.parse().unwrap();
}
