use huff_lexer::*;
use huff_parser::*;
use huff_utils::{ast::EventDefinition, prelude::*};

#[test]
fn test_prefix_event_arg_names_with_reserved_keywords() {
    let source: &str =  "#define event TestEvent(bytes4 indexed interfaceId, uint256 uintTest, bool stringMe, string boolean)";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let expected_tokens: Vec<Token> = vec![
        Token { kind: TokenKind::Define, span: Span { start: 0, end: 6, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 7, end: 7, file: None } },
        Token { kind: TokenKind::Event, span: Span { start: 8, end: 12, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 13, end: 13, file: None } },
        Token {
            kind: TokenKind::Ident("TestEvent".to_string()),
            span: Span { start: 14, end: 22, file: None },
        },
        Token { kind: TokenKind::OpenParen, span: Span { start: 23, end: 23, file: None } },
        Token {
            kind: TokenKind::PrimitiveType(PrimitiveEVMType::Bytes(4)),
            span: Span { start: 24, end: 29, file: None },
        },
        Token { kind: TokenKind::Whitespace, span: Span { start: 30, end: 30, file: None } },
        Token { kind: TokenKind::Indexed, span: Span { start: 31, end: 37, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 38, end: 38, file: None } },
        Token {
            kind: TokenKind::Ident("interfaceId".to_string()),
            span: Span { start: 39, end: 49, file: None },
        },
        Token { kind: TokenKind::Comma, span: Span { start: 50, end: 50, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 51, end: 51, file: None } },
        Token {
            kind: TokenKind::PrimitiveType(PrimitiveEVMType::Uint(256)),
            span: Span { start: 52, end: 58, file: None },
        },
        Token { kind: TokenKind::Whitespace, span: Span { start: 59, end: 59, file: None } },
        Token {
            kind: TokenKind::Ident("uintTest".to_string()),
            span: Span { start: 60, end: 67, file: None },
        },
        Token { kind: TokenKind::Comma, span: Span { start: 68, end: 68, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 69, end: 69, file: None } },
        Token {
            kind: TokenKind::PrimitiveType(PrimitiveEVMType::Bool),
            span: Span { start: 70, end: 73, file: None },
        },
        Token { kind: TokenKind::Whitespace, span: Span { start: 74, end: 74, file: None } },
        Token {
            kind: TokenKind::Ident("stringMe".to_string()),
            span: Span { start: 75, end: 82, file: None },
        },
        Token { kind: TokenKind::Comma, span: Span { start: 83, end: 83, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 84, end: 84, file: None } },
        Token {
            kind: TokenKind::PrimitiveType(PrimitiveEVMType::String),
            span: Span { start: 85, end: 90, file: None },
        },
        Token { kind: TokenKind::Whitespace, span: Span { start: 91, end: 91, file: None } },
        Token {
            kind: TokenKind::Ident("boolean".to_string()),
            span: Span { start: 92, end: 98, file: None },
        },
        Token { kind: TokenKind::CloseParen, span: Span { start: 99, end: 99, file: None } },
        Token { kind: TokenKind::Eof, span: Span { start: 99, end: 99, file: None } },
    ];
    assert_eq!(expected_tokens, tokens);
    let mut parser = Parser::new(tokens, None);
    parser.parse().unwrap();
}

#[test]
fn test_parse_event() {
    let sources = [
        (
            "#define event TestEvent(uint256 indexed a,uint8 indexed)",
            EventDefinition {
                name: "TestEvent".to_string(),
                parameters: vec![
                    Argument {
                        arg_type: Some(String::from("uint256")),
                        name: Some(String::from("a")),
                        indexed: true,
                        arg_location: None,
                        span: AstSpan(vec![
                            // "uint256"
                            Span { start: 24, end: 30, file: None },
                            // "indexed"
                            Span { start: 32, end: 38, file: None },
                            // "a"
                            Span { start: 40, end: 40, file: None },
                        ]),
                    },
                    Argument {
                        arg_type: Some(String::from("uint8")),
                        name: None,
                        indexed: true,
                        arg_location: None,
                        span: AstSpan(vec![
                            // "uint8"
                            Span { start: 42, end: 46, file: None },
                            // "indexed"
                            Span { start: 48, end: 54, file: None },
                        ]),
                    },
                ],
                span: AstSpan(vec![
                    // "#define"
                    Span { start: 0, end: 6, file: None },
                    // "event"
                    Span { start: 8, end: 12, file: None },
                    // "TestEvent"
                    Span { start: 14, end: 22, file: None },
                    // "("
                    Span { start: 23, end: 23, file: None },
                    // "uint256"
                    Span { start: 24, end: 30, file: None },
                    // "indexed"
                    Span { start: 32, end: 38, file: None },
                    // "a"
                    Span { start: 40, end: 40, file: None },
                    // ","
                    Span { start: 41, end: 41, file: None },
                    // "uint8"
                    Span { start: 42, end: 46, file: None },
                    // "indexed"
                    Span { start: 48, end: 54, file: None },
                    // ")"
                    Span { start: 55, end: 55, file: None },
                ]),
                hash: [
                    96, 60, 104, 14, 131, 197, 151, 167, 15, 107, 26, 61, 1, 186, 238, 67, 62, 152,
                    177, 8, 184, 82, 242, 224, 18, 70, 210, 27, 27, 119, 23, 114,
                ],
            },
        ),
        (
            "#define event TestEvent(uint256,uint8 b)",
            EventDefinition {
                name: "TestEvent".to_string(),
                parameters: vec![
                    Argument {
                        arg_type: Some(String::from("uint256")),
                        name: None,
                        indexed: false,
                        arg_location: None,
                        span: AstSpan(vec![
                            // "uint256"
                            Span { start: 24, end: 30, file: None },
                        ]),
                    },
                    Argument {
                        arg_type: Some(String::from("uint8")),
                        name: Some(String::from("b")),
                        indexed: false,
                        arg_location: None,
                        span: AstSpan(vec![
                            // "uint8"
                            Span { start: 32, end: 36, file: None },
                            // "b"
                            Span { start: 38, end: 38, file: None },
                        ]),
                    },
                ],
                span: AstSpan(vec![
                    // "#define"
                    Span { start: 0, end: 6, file: None },
                    // "event"
                    Span { start: 8, end: 12, file: None },
                    // "TestEvent"
                    Span { start: 14, end: 22, file: None },
                    // "("
                    Span { start: 23, end: 23, file: None },
                    // "uint256"
                    Span { start: 24, end: 30, file: None },
                    // ","
                    Span { start: 31, end: 31, file: None },
                    // "uint8"
                    Span { start: 32, end: 36, file: None },
                    // "b"
                    Span { start: 38, end: 38, file: None },
                    // ")"
                    Span { start: 39, end: 39, file: None },
                ]),
                hash: [
                    96, 60, 104, 14, 131, 197, 151, 167, 15, 107, 26, 61, 1, 186, 238, 67, 62, 152,
                    177, 8, 184, 82, 242, 224, 18, 70, 210, 27, 27, 119, 23, 114,
                ],
            },
        ),
        (
            "#define event TestEvent(uint256 indexed,uint8)",
            EventDefinition {
                name: "TestEvent".to_string(),
                parameters: vec![
                    Argument {
                        arg_type: Some(String::from("uint256")),
                        name: None,
                        indexed: true,
                        arg_location: None,
                        span: AstSpan(vec![
                            // "uint256"
                            Span { start: 24, end: 30, file: None },
                            // "indexed"
                            Span { start: 32, end: 38, file: None },
                        ]),
                    },
                    Argument {
                        arg_type: Some(String::from("uint8")),
                        name: None,
                        indexed: false,
                        arg_location: None,
                        span: AstSpan(vec![
                            // "uint8"
                            Span { start: 40, end: 44, file: None },
                        ]),
                    },
                ],
                span: AstSpan(vec![
                    // "#define"
                    Span { start: 0, end: 6, file: None },
                    // "event"
                    Span { start: 8, end: 12, file: None },
                    // "TestEvent"
                    Span { start: 14, end: 22, file: None },
                    // "("
                    Span { start: 23, end: 23, file: None },
                    // "uint256"
                    Span { start: 24, end: 30, file: None },
                    // "indexed"
                    Span { start: 32, end: 38, file: None },
                    // ","
                    Span { start: 39, end: 39, file: None },
                    // "uint8"
                    Span { start: 40, end: 44, file: None },
                    // ")"
                    Span { start: 45, end: 45, file: None },
                ]),
                hash: [
                    96, 60, 104, 14, 131, 197, 151, 167, 15, 107, 26, 61, 1, 186, 238, 67, 62, 152,
                    177, 8, 184, 82, 242, 224, 18, 70, 210, 27, 27, 119, 23, 114,
                ],
            },
        ),
    ];

    for (source, expected) in sources {
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(flattened_source.source);
        let tokens = lexer
            .into_iter()
            .map(|x| x.unwrap())
            .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
            .collect::<Vec<Token>>();
        let mut parser = Parser::new(tokens, None);
        let _ = parser.match_kind(TokenKind::Define);
        let event = parser.parse_event().unwrap();

        assert_eq!(event, expected);
    }
}
