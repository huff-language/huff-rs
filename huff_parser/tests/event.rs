use huff_lexer::*;
use huff_parser::*;
use huff_utils::{ast::Event, prelude::*};

#[test]
fn parse_event() {
    let sources = [
        (
            "#define event TestEvent(uint256 indexed a,uint8 indexed)",
            Event {
                name: "TestEvent",
                parameters: vec![
                    Argument {
                        arg_type: Some(String::from("uint256")),
                        name: Some(String::from("a")),
                        indexed: true,
                    },
                    Argument { arg_type: Some(String::from("uint8")), name: None, indexed: true },
                ],
            },
        ),
        (
            "#define event TestEvent(uint256,uint8 b)",
            Event {
                name: "TestEvent",
                parameters: vec![
                    Argument {
                        arg_type: Some(String::from("uint256")),
                        name: None,
                        indexed: false,
                    },
                    Argument {
                        arg_type: Some(String::from("uint8")),
                        name: Some(String::from("b")),
                        indexed: false,
                    },
                ],
            },
        ),
        (
            "#define event TestEvent(uint256 indexed,uint8)",
            Event {
                name: "TestEvent",
                parameters: vec![
                    Argument { arg_type: Some(String::from("uint256")), name: None, indexed: true },
                    Argument { arg_type: Some(String::from("uint8")), name: None, indexed: false },
                ],
            },
        ),
    ];

    for (source, expected) in sources {
        let lexer = Lexer::new(source);
        let tokens = lexer
            .into_iter()
            .map(|x| x.unwrap())
            .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
            .collect::<Vec<Token>>();
        let mut parser = Parser::new(tokens);
        let _ = parser.match_kind(TokenKind::Define);
        let event = parser.parse_event().unwrap();

        assert_eq!(event, expected);
    }
}
