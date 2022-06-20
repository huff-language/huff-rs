use huff_lexer::*;
use huff_parser::*;
use huff_utils::{ast::Event, prelude::*};

#[test]
fn test_parse_event() {
    let sources = [
        (
            "#define event TestEvent(uint256 indexed a,uint8 indexed)",
            Event {
                name: "TestEvent".to_string(),
                parameters: vec![
                    Argument {
                        arg_type: Some(String::from("uint256")),
                        name: Some(String::from("a")),
                        indexed: true,
                        span: AstSpan(vec![
                            // "uint256"
                            Span { start: 24, end: 31, file: None },
                            // "indexed"
                            Span { start: 32, end: 39, file: None },
                            // "a"
                            Span { start: 40, end: 41, file: None },
                        ]),
                    },
                    Argument {
                        arg_type: Some(String::from("uint8")),
                        name: None,
                        indexed: true,
                        span: AstSpan(vec![
                            // "uint8"
                            Span { start: 42, end: 47, file: None },
                            // "indexed"
                            Span { start: 48, end: 55, file: None },
                        ]),
                    },
                ],
                span: AstSpan(vec![
                    // "#define"
                    Span { start: 0, end: 7, file: None },
                    // "event"
                    Span { start: 8, end: 13, file: None },
                    // "TestEvent"
                    Span { start: 14, end: 23, file: None },
                    // "("
                    Span { start: 23, end: 24, file: None },
                    // "uint256"
                    Span { start: 24, end: 31, file: None },
                    // "indexed"
                    Span { start: 32, end: 39, file: None },
                    // "a"
                    Span { start: 40, end: 41, file: None },
                    // ","
                    Span { start: 41, end: 42, file: None },
                    // "uint8"
                    Span { start: 42, end: 47, file: None },
                    // "indexed"
                    Span { start: 48, end: 55, file: None },
                    // ")"
                    Span { start: 55, end: 56, file: None },
                ]),
            },
        ),
        (
            "#define event TestEvent(uint256,uint8 b)",
            Event {
                name: "TestEvent".to_string(),
                parameters: vec![
                    Argument {
                        arg_type: Some(String::from("uint256")),
                        name: None,
                        indexed: false,
                        span: AstSpan(vec![
                            // "uint256"
                            Span { start: 24, end: 31, file: None },
                        ]),
                    },
                    Argument {
                        arg_type: Some(String::from("uint8")),
                        name: Some(String::from("b")),
                        indexed: false,
                        span: AstSpan(vec![
                            // "uint8"
                            Span { start: 32, end: 37, file: None },
                            // "b"
                            Span { start: 38, end: 39, file: None },
                        ]),
                    },
                ],
                span: AstSpan(vec![
                    // "#define"
                    Span { start: 0, end: 7, file: None },
                    // "event"
                    Span { start: 8, end: 13, file: None },
                    // "TestEvent"
                    Span { start: 14, end: 23, file: None },
                    // "("
                    Span { start: 23, end: 24, file: None },
                    // "uint256"
                    Span { start: 24, end: 31, file: None },
                    // ","
                    Span { start: 31, end: 32, file: None },
                    // "uint8"
                    Span { start: 32, end: 37, file: None },
                    // "b"
                    Span { start: 38, end: 39, file: None },
                    // ")"
                    Span { start: 39, end: 40, file: None },
                ]),
            },
        ),
        (
            "#define event TestEvent(uint256 indexed,uint8)",
            Event {
                name: "TestEvent".to_string(),
                parameters: vec![
                    Argument {
                        arg_type: Some(String::from("uint256")),
                        name: None,
                        indexed: true,
                        span: AstSpan(vec![
                            // "uint256"
                            Span { start: 24, end: 31, file: None },
                            // "indexed"
                            Span { start: 32, end: 39, file: None },
                        ]),
                    },
                    Argument {
                        arg_type: Some(String::from("uint8")),
                        name: None,
                        indexed: false,
                        span: AstSpan(vec![
                            // "uint8"
                            Span { start: 40, end: 45, file: None },
                        ]),
                    },
                ],
                span: AstSpan(vec![
                    // "#define"
                    Span { start: 0, end: 7, file: None },
                    // "event"
                    Span { start: 8, end: 13, file: None },
                    // "TestEvent"
                    Span { start: 14, end: 23, file: None },
                    // "("
                    Span { start: 23, end: 24, file: None },
                    // "uint256"
                    Span { start: 24, end: 31, file: None },
                    // "indexed"
                    Span { start: 32, end: 39, file: None },
                    // ","
                    Span { start: 39, end: 40, file: None },
                    // "uint8"
                    Span { start: 40, end: 45, file: None },
                    // ")"
                    Span { start: 45, end: 46, file: None },
                ]),
            },
        ),
    ];

    for (source, expected) in sources {
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(flattened_source);
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
