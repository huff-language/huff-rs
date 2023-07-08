use huff_lexer::Lexer;
use huff_parser::*;
use huff_utils::{evm::Opcode, prelude::*};

#[test]
fn empty_macro() {
    let source = "#define macro HELLO_WORLD() = takes(0) returns(4) {}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        decorator: None,
        parameters: vec![],
        statements: vec![],
        takes: 0,
        returns: 4,
        span: AstSpan(vec![
            Span { start: 0, end: 6, file: None },
            Span { start: 8, end: 12, file: None },
            Span { start: 14, end: 24, file: None },
            Span { start: 25, end: 25, file: None },
            Span { start: 26, end: 26, file: None },
            Span { start: 28, end: 28, file: None },
            Span { start: 30, end: 34, file: None },
            Span { start: 35, end: 35, file: None },
            Span { start: 36, end: 36, file: None },
            Span { start: 37, end: 37, file: None },
            Span { start: 39, end: 45, file: None },
            Span { start: 46, end: 46, file: None },
            Span { start: 47, end: 47, file: None },
            Span { start: 48, end: 48, file: None },
            Span { start: 50, end: 50, file: None },
            Span { start: 51, end: 51, file: None },
        ]),
        outlined: false,
        test: false,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn empty_macro_without_takes_returns() {
    let source = "#define macro HELLO_WORLD() = {}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        decorator: None,
        parameters: vec![],
        statements: vec![],
        takes: 0,
        returns: 0,
        span: AstSpan(vec![
            Span { start: 0, end: 6, file: None },
            Span { start: 8, end: 12, file: None },
            Span { start: 14, end: 24, file: None },
            Span { start: 25, end: 25, file: None },
            Span { start: 26, end: 26, file: None },
            Span { start: 28, end: 28, file: None },
            Span { start: 30, end: 30, file: None },
            Span { start: 31, end: 31, file: None },
        ]),
        outlined: false,
        test: false,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn empty_macro_only_takes() {
    let source = "#define macro HELLO_WORLD() = takes(3) {}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        decorator: None,
        parameters: vec![],
        statements: vec![],
        takes: 3,
        returns: 0,
        span: AstSpan(vec![
            Span { start: 0, end: 6, file: None },
            Span { start: 8, end: 12, file: None },
            Span { start: 14, end: 24, file: None },
            Span { start: 25, end: 25, file: None },
            Span { start: 26, end: 26, file: None },
            Span { start: 28, end: 28, file: None },
            Span { start: 30, end: 34, file: None },
            Span { start: 35, end: 35, file: None },
            Span { start: 36, end: 36, file: None },
            Span { start: 37, end: 37, file: None },
            Span { start: 39, end: 39, file: None },
            Span { start: 40, end: 40, file: None },
        ]),
        outlined: false,
        test: false,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn empty_macro_only_returns() {
    let source = "#define macro HELLO_WORLD() = returns(10) {}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        decorator: None,
        parameters: vec![],
        statements: vec![],
        takes: 0,
        returns: 10,
        span: AstSpan(vec![
            Span { start: 0, end: 6, file: None },
            Span { start: 8, end: 12, file: None },
            Span { start: 14, end: 24, file: None },
            Span { start: 25, end: 25, file: None },
            Span { start: 26, end: 26, file: None },
            Span { start: 28, end: 28, file: None },
            Span { start: 30, end: 36, file: None },
            Span { start: 37, end: 37, file: None },
            Span { start: 38, end: 39, file: None },
            Span { start: 40, end: 40, file: None },
            Span { start: 42, end: 42, file: None },
            Span { start: 43, end: 43, file: None },
        ]),
        outlined: false,
        test: false,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn macro_with_simple_body() {
    let source =
        "#define macro HELLO_WORLD() = takes(3) returns(0) {\n0x00 mstore\n 0x01 0x02 add\n}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        decorator: None,
        parameters: vec![],
        statements: vec![
            Statement {
                ty: StatementType::Literal(str_to_bytes32("00")),
                span: AstSpan(vec![Span { start: 54, end: 55, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Mstore),
                span: AstSpan(vec![Span { start: 57, end: 62, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("01")),
                span: AstSpan(vec![Span { start: 67, end: 68, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("02")),
                span: AstSpan(vec![Span { start: 72, end: 73, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Add),
                span: AstSpan(vec![Span { start: 75, end: 77, file: None }]),
            },
        ],
        takes: 3,
        returns: 0,
        span: AstSpan(vec![
            Span { start: 0, end: 6, file: None },
            Span { start: 8, end: 12, file: None },
            Span { start: 14, end: 24, file: None },
            Span { start: 25, end: 25, file: None },
            Span { start: 26, end: 26, file: None },
            Span { start: 28, end: 28, file: None },
            Span { start: 30, end: 34, file: None },
            Span { start: 35, end: 35, file: None },
            Span { start: 36, end: 36, file: None },
            Span { start: 37, end: 37, file: None },
            Span { start: 39, end: 45, file: None },
            Span { start: 46, end: 46, file: None },
            Span { start: 47, end: 47, file: None },
            Span { start: 48, end: 48, file: None },
            Span { start: 50, end: 50, file: None },
            Span { start: 54, end: 55, file: None },
            Span { start: 57, end: 62, file: None },
            Span { start: 67, end: 68, file: None },
            Span { start: 72, end: 73, file: None },
            Span { start: 75, end: 77, file: None },
            Span { start: 79, end: 79, file: None },
        ]),
        outlined: false,
        test: false,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn macro_with_arg_calls() {
    let source = r#"
    /* Transfer Functions */
    #define macro TRANSFER_TAKE_FROM(error) = takes(3) returns (3) {
        // Ensure that the sender has a sufficient balance.
        // input stack: [value, from, to]
        dup2                // [from, value, from, to]
        [BALANCE_LOCATION] LOAD_ELEMENT_FROM_KEYS(0x00)  // [balance, value, from, to]
        dup1                // [balance, balance, value, from, to]
        dup3                // [value, balance, balance, value, from, to]
        gt                  // [value>balance, balance, value, from, to]
        <error> jumpi       // [balance, value, from, to]

        // Update the sender's balance.
        // input stack: [balance, value, from, to]
        dup2                // [value, balance, value, from, to]
        swap1               // [balance, value, value, from, to]
        sub                 // [balance - value, value, from, to]
        dup3                // [from, balance-value, value, from, to]
        [BALANCE_LOCATION] STORE_ELEMENT_FROM_KEYS(0x00) // [value, from, to]
    }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "TRANSFER_TAKE_FROM".to_string(),
        parameters: vec![Argument {
            arg_type: None,
            name: Some("error".to_string()),
            indexed: false,
            arg_location: None,
            span: AstSpan(vec![Span { start: 67, end: 71, file: None }]),
        }],
        decorator: None,
        statements: vec![
            Statement {
                ty: StatementType::Opcode(Opcode::Dup2),
                span: AstSpan(vec![Span { start: 209, end: 212, file: None }]),
            },
            Statement {
                ty: StatementType::Constant("BALANCE_LOCATION".to_string()),
                span: AstSpan(vec![Span { start: 265, end: 280, file: None }]),
            },
            Statement {
                ty: StatementType::MacroInvocation(MacroInvocation {
                    macro_name: "LOAD_ELEMENT_FROM_KEYS".to_string(),
                    args: vec![MacroArg::Literal(str_to_bytes32("00"))],
                    span: AstSpan(vec![
                        Span { start: 283, end: 304, file: None },
                        Span { start: 305, end: 305, file: None },
                        Span { start: 308, end: 309, file: None },
                        Span { start: 310, end: 310, file: None },
                    ]),
                }),
                span: AstSpan(vec![
                    Span { start: 283, end: 304, file: None },
                    Span { start: 305, end: 305, file: None },
                    Span { start: 308, end: 309, file: None },
                    Span { start: 310, end: 310, file: None },
                ]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Dup1),
                span: AstSpan(vec![Span { start: 351, end: 354, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Dup3),
                span: AstSpan(vec![Span { start: 418, end: 421, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Gt),
                span: AstSpan(vec![Span { start: 492, end: 493, file: None }]),
            },
            Statement {
                ty: StatementType::ArgCall("error".to_string()),
                span: AstSpan(vec![Span { start: 566, end: 570, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Jumpi),
                span: AstSpan(vec![Span { start: 573, end: 577, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Dup2),
                span: AstSpan(vec![Span { start: 715, end: 718, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Swap1),
                span: AstSpan(vec![Span { start: 780, end: 784, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Sub),
                span: AstSpan(vec![Span { start: 845, end: 847, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Dup3),
                span: AstSpan(vec![Span { start: 911, end: 914, file: None }]),
            },
            Statement {
                ty: StatementType::Constant("BALANCE_LOCATION".to_string()),
                span: AstSpan(vec![Span { start: 982, end: 997, file: None }]),
            },
            Statement {
                ty: StatementType::MacroInvocation(MacroInvocation {
                    macro_name: "STORE_ELEMENT_FROM_KEYS".to_string(),
                    args: vec![MacroArg::Literal(str_to_bytes32("00"))],
                    span: AstSpan(vec![
                        Span { start: 1000, end: 1022, file: None },
                        Span { start: 1023, end: 1023, file: None },
                        Span { start: 1026, end: 1027, file: None },
                        Span { start: 1028, end: 1028, file: None },
                    ]),
                }),
                span: AstSpan(vec![
                    Span { start: 1000, end: 1022, file: None },
                    Span { start: 1023, end: 1023, file: None },
                    Span { start: 1026, end: 1027, file: None },
                    Span { start: 1028, end: 1028, file: None },
                ]),
            },
        ],
        takes: 3,
        returns: 3,
        span: AstSpan(vec![
            Span { start: 34, end: 40, file: None },
            Span { start: 42, end: 46, file: None },
            Span { start: 48, end: 65, file: None },
            Span { start: 66, end: 66, file: None },
            Span { start: 67, end: 71, file: None },
            Span { start: 72, end: 72, file: None },
            Span { start: 74, end: 74, file: None },
            Span { start: 76, end: 80, file: None },
            Span { start: 81, end: 81, file: None },
            Span { start: 82, end: 82, file: None },
            Span { start: 83, end: 83, file: None },
            Span { start: 85, end: 91, file: None },
            Span { start: 93, end: 93, file: None },
            Span { start: 94, end: 94, file: None },
            Span { start: 95, end: 95, file: None },
            Span { start: 97, end: 97, file: None },
            Span { start: 209, end: 212, file: None },
            Span { start: 264, end: 264, file: None },
            Span { start: 265, end: 280, file: None },
            Span { start: 281, end: 281, file: None },
            Span { start: 283, end: 304, file: None },
            Span { start: 305, end: 305, file: None },
            Span { start: 308, end: 309, file: None },
            Span { start: 310, end: 310, file: None },
            Span { start: 351, end: 354, file: None },
            Span { start: 418, end: 421, file: None },
            Span { start: 492, end: 493, file: None },
            Span { start: 565, end: 565, file: None },
            Span { start: 566, end: 570, file: None },
            Span { start: 571, end: 571, file: None },
            Span { start: 573, end: 577, file: None },
            Span { start: 715, end: 718, file: None },
            Span { start: 780, end: 784, file: None },
            Span { start: 845, end: 847, file: None },
            Span { start: 911, end: 914, file: None },
            Span { start: 981, end: 981, file: None },
            Span { start: 982, end: 997, file: None },
            Span { start: 998, end: 998, file: None },
            Span { start: 1000, end: 1022, file: None },
            Span { start: 1023, end: 1023, file: None },
            Span { start: 1026, end: 1027, file: None },
            Span { start: 1028, end: 1028, file: None },
            Span { start: 1055, end: 1055, file: None },
        ]),
        outlined: false,
        test: false,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn macro_labels() {
    let source = r#"
    #define macro LABEL_FILLED() = takes(0) returns(0) {
        __label__:
            TRANSFER_GIVE_TO()
            0x00 0x00 revert
        error:
            TRANSFER_GIVE_TO()
            0x00 0x00 revert
    }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    // let lexer = Lexer::new(flattened_source.source);
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "LABEL_FILLED".to_string(),
        decorator: None,
        parameters: vec![],
        statements: vec![
            Statement {
                ty: StatementType::Label(Label {
                    name: "__label__".to_string(),
                    inner: vec![
                        Statement {
                            ty: StatementType::MacroInvocation(MacroInvocation {
                                macro_name: "TRANSFER_GIVE_TO".to_string(),
                                args: vec![],
                                span: AstSpan(vec![
                                    Span { start: 89, end: 104, file: None },
                                    Span { start: 105, end: 105, file: None },
                                    Span { start: 106, end: 106, file: None },
                                ]),
                            }),
                            span: AstSpan(vec![
                                Span { start: 89, end: 104, file: None },
                                Span { start: 105, end: 105, file: None },
                                Span { start: 106, end: 106, file: None },
                            ]),
                        },
                        Statement {
                            ty: StatementType::Literal(str_to_bytes32("00")),
                            span: AstSpan(vec![Span { start: 122, end: 123, file: None }]),
                        },
                        Statement {
                            ty: StatementType::Literal(str_to_bytes32("00")),
                            span: AstSpan(vec![Span { start: 127, end: 128, file: None }]),
                        },
                        Statement {
                            ty: StatementType::Opcode(Opcode::Revert),
                            span: AstSpan(vec![Span { start: 130, end: 135, file: None }]),
                        },
                    ],
                    span: AstSpan(vec![
                        Span { start: 66, end: 74, file: None },
                        Span { start: 89, end: 104, file: None },
                        Span { start: 105, end: 105, file: None },
                        Span { start: 106, end: 106, file: None },
                        Span { start: 122, end: 123, file: None },
                        Span { start: 127, end: 128, file: None },
                        Span { start: 130, end: 135, file: None },
                    ]),
                }),
                span: AstSpan(vec![
                    Span { start: 66, end: 74, file: None },
                    Span { start: 89, end: 104, file: None },
                    Span { start: 105, end: 105, file: None },
                    Span { start: 106, end: 106, file: None },
                    Span { start: 122, end: 123, file: None },
                    Span { start: 127, end: 128, file: None },
                    Span { start: 130, end: 135, file: None },
                ]),
            },
            Statement {
                ty: StatementType::Label(Label {
                    name: "error".to_string(),
                    inner: vec![
                        Statement {
                            ty: StatementType::MacroInvocation(MacroInvocation {
                                macro_name: "TRANSFER_GIVE_TO".to_string(),
                                args: vec![],
                                span: AstSpan(vec![
                                    Span { start: 164, end: 179, file: None },
                                    Span { start: 180, end: 180, file: None },
                                    Span { start: 181, end: 181, file: None },
                                ]),
                            }),
                            span: AstSpan(vec![
                                Span { start: 164, end: 179, file: None },
                                Span { start: 180, end: 180, file: None },
                                Span { start: 181, end: 181, file: None },
                            ]),
                        },
                        Statement {
                            ty: StatementType::Literal(str_to_bytes32("00")),
                            span: AstSpan(vec![Span { start: 197, end: 198, file: None }]),
                        },
                        Statement {
                            ty: StatementType::Literal(str_to_bytes32("00")),
                            span: AstSpan(vec![Span { start: 202, end: 203, file: None }]),
                        },
                        Statement {
                            ty: StatementType::Opcode(Opcode::Revert),
                            span: AstSpan(vec![Span { start: 205, end: 210, file: None }]),
                        },
                    ],
                    span: AstSpan(vec![
                        Span { start: 145, end: 149, file: None },
                        Span { start: 164, end: 179, file: None },
                        Span { start: 180, end: 180, file: None },
                        Span { start: 181, end: 181, file: None },
                        Span { start: 197, end: 198, file: None },
                        Span { start: 202, end: 203, file: None },
                        Span { start: 205, end: 210, file: None },
                    ]),
                }),
                span: AstSpan(vec![
                    Span { start: 145, end: 149, file: None },
                    Span { start: 164, end: 179, file: None },
                    Span { start: 180, end: 180, file: None },
                    Span { start: 181, end: 181, file: None },
                    Span { start: 197, end: 198, file: None },
                    Span { start: 202, end: 203, file: None },
                    Span { start: 205, end: 210, file: None },
                ]),
            },
        ],
        takes: 0,
        returns: 0,
        span: AstSpan(vec![
            Span { start: 5, end: 11, file: None },
            Span { start: 13, end: 17, file: None },
            Span { start: 19, end: 30, file: None },
            Span { start: 31, end: 31, file: None },
            Span { start: 32, end: 32, file: None },
            Span { start: 34, end: 34, file: None },
            Span { start: 36, end: 40, file: None },
            Span { start: 41, end: 41, file: None },
            Span { start: 42, end: 42, file: None },
            Span { start: 43, end: 43, file: None },
            Span { start: 45, end: 51, file: None },
            Span { start: 52, end: 52, file: None },
            Span { start: 53, end: 53, file: None },
            Span { start: 54, end: 54, file: None },
            Span { start: 56, end: 56, file: None },
            Span { start: 66, end: 74, file: None },
            Span { start: 75, end: 75, file: None },
            Span { start: 89, end: 104, file: None },
            Span { start: 105, end: 105, file: None },
            Span { start: 106, end: 106, file: None },
            Span { start: 122, end: 123, file: None },
            Span { start: 127, end: 128, file: None },
            Span { start: 130, end: 135, file: None },
            Span { start: 145, end: 149, file: None },
            Span { start: 150, end: 150, file: None },
            Span { start: 164, end: 179, file: None },
            Span { start: 180, end: 180, file: None },
            Span { start: 181, end: 181, file: None },
            Span { start: 197, end: 198, file: None },
            Span { start: 202, end: 203, file: None },
            Span { start: 205, end: 210, file: None },
            Span { start: 216, end: 216, file: None },
        ]),
        outlined: false,
        test: false,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn macro_invocation_with_arg_call() {
    let source = r#"
    #define macro ARG_CALL(error) = takes(0) returns(0) {
        TRANSFER_TAKE_FROM(<error>)
        TRANSFER_GIVE_TO(<error>)

        0x01 0x00 mstore
        0x20 0x00 return
    }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "ARG_CALL".to_string(),
        decorator: None,
        parameters: vec![Argument {
            arg_type: None,
            name: Some("error".to_string()),
            indexed: false,
            arg_location: None,
            span: AstSpan(vec![Span { start: 28, end: 32, file: None }]),
        }],
        statements: vec![
            Statement {
                ty: StatementType::MacroInvocation(MacroInvocation {
                    macro_name: "TRANSFER_TAKE_FROM".to_string(),
                    args: vec![MacroArg::ArgCall("error".to_string())],
                    span: AstSpan(vec![
                        Span { start: 67, end: 84, file: None },
                        Span { start: 85, end: 85, file: None },
                        Span { start: 86, end: 86, file: None },
                        Span { start: 87, end: 91, file: None },
                        Span { start: 92, end: 92, file: None },
                        Span { start: 93, end: 93, file: None },
                    ]),
                }),
                span: AstSpan(vec![
                    Span { start: 67, end: 84, file: None },
                    Span { start: 85, end: 85, file: None },
                    Span { start: 86, end: 86, file: None },
                    Span { start: 87, end: 91, file: None },
                    Span { start: 92, end: 92, file: None },
                    Span { start: 93, end: 93, file: None },
                ]),
            },
            Statement {
                ty: StatementType::MacroInvocation(MacroInvocation {
                    macro_name: "TRANSFER_GIVE_TO".to_string(),
                    args: vec![MacroArg::ArgCall("error".to_string())],
                    span: AstSpan(vec![
                        Span { start: 103, end: 118, file: None },
                        Span { start: 119, end: 119, file: None },
                        Span { start: 120, end: 120, file: None },
                        Span { start: 121, end: 125, file: None },
                        Span { start: 126, end: 126, file: None },
                        Span { start: 127, end: 127, file: None },
                    ]),
                }),
                span: AstSpan(vec![
                    Span { start: 103, end: 118, file: None },
                    Span { start: 119, end: 119, file: None },
                    Span { start: 120, end: 120, file: None },
                    Span { start: 121, end: 125, file: None },
                    Span { start: 126, end: 126, file: None },
                    Span { start: 127, end: 127, file: None },
                ]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("01")),
                span: AstSpan(vec![Span { start: 140, end: 141, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("00")),
                span: AstSpan(vec![Span { start: 145, end: 146, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Mstore),
                span: AstSpan(vec![Span { start: 148, end: 153, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("20")),
                span: AstSpan(vec![Span { start: 165, end: 166, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("00")),
                span: AstSpan(vec![Span { start: 170, end: 171, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Return),
                span: AstSpan(vec![Span { start: 173, end: 178, file: None }]),
            },
        ],
        takes: 0,
        returns: 0,
        span: AstSpan(vec![
            Span { start: 5, end: 11, file: None },
            Span { start: 13, end: 17, file: None },
            Span { start: 19, end: 26, file: None },
            Span { start: 27, end: 27, file: None },
            Span { start: 28, end: 32, file: None },
            Span { start: 33, end: 33, file: None },
            Span { start: 35, end: 35, file: None },
            Span { start: 37, end: 41, file: None },
            Span { start: 42, end: 42, file: None },
            Span { start: 43, end: 43, file: None },
            Span { start: 44, end: 44, file: None },
            Span { start: 46, end: 52, file: None },
            Span { start: 53, end: 53, file: None },
            Span { start: 54, end: 54, file: None },
            Span { start: 55, end: 55, file: None },
            Span { start: 57, end: 57, file: None },
            Span { start: 67, end: 84, file: None },
            Span { start: 85, end: 85, file: None },
            Span { start: 86, end: 86, file: None },
            Span { start: 87, end: 91, file: None },
            Span { start: 92, end: 92, file: None },
            Span { start: 93, end: 93, file: None },
            Span { start: 103, end: 118, file: None },
            Span { start: 119, end: 119, file: None },
            Span { start: 120, end: 120, file: None },
            Span { start: 121, end: 125, file: None },
            Span { start: 126, end: 126, file: None },
            Span { start: 127, end: 127, file: None },
            Span { start: 140, end: 141, file: None },
            Span { start: 145, end: 146, file: None },
            Span { start: 148, end: 153, file: None },
            Span { start: 165, end: 166, file: None },
            Span { start: 170, end: 171, file: None },
            Span { start: 173, end: 178, file: None },
            Span { start: 184, end: 184, file: None },
        ]),
        outlined: false,
        test: false,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn test_macro_opcode_arguments() {
    let source = r#"
    #define macro MAIN() = takes(0) returns(0) {
        RETURN1(returndatasize)
    }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "MAIN".to_string(),
        decorator: None,
        parameters: vec![],
        statements: vec![Statement {
            ty: StatementType::MacroInvocation(MacroInvocation {
                macro_name: "RETURN1".to_string(),
                args: vec![MacroArg::Ident("returndatasize".to_string())],
                span: AstSpan(vec![
                    Span { start: 58, end: 64, file: None },
                    Span { start: 65, end: 65, file: None },
                    Span { start: 66, end: 79, file: None },
                    Span { start: 80, end: 80, file: None },
                ]),
            }),
            span: AstSpan(vec![
                Span { start: 58, end: 64, file: None },
                Span { start: 65, end: 65, file: None },
                Span { start: 66, end: 79, file: None },
                Span { start: 80, end: 80, file: None },
            ]),
        }],
        takes: 0,
        returns: 0,
        span: AstSpan(vec![
            Span { start: 5, end: 11, file: None },
            Span { start: 13, end: 17, file: None },
            Span { start: 19, end: 22, file: None },
            Span { start: 23, end: 23, file: None },
            Span { start: 24, end: 24, file: None },
            Span { start: 26, end: 26, file: None },
            Span { start: 28, end: 32, file: None },
            Span { start: 33, end: 33, file: None },
            Span { start: 34, end: 34, file: None },
            Span { start: 35, end: 35, file: None },
            Span { start: 37, end: 43, file: None },
            Span { start: 44, end: 44, file: None },
            Span { start: 45, end: 45, file: None },
            Span { start: 46, end: 46, file: None },
            Span { start: 48, end: 48, file: None },
            Span { start: 58, end: 64, file: None },
            Span { start: 65, end: 65, file: None },
            Span { start: 66, end: 79, file: None },
            Span { start: 80, end: 80, file: None },
            Span { start: 86, end: 86, file: None },
        ]),
        outlined: false,
        test: false,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn macro_with_builtin_fn_call() {
    // Not valid source, just for testing
    let source = r#"
    #define macro BUILTIN_TEST() = takes(0) returns(0) {
        __codesize(TEST)
    }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);

    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "BUILTIN_TEST".to_string(),
        decorator: None,
        parameters: vec![],
        statements: vec![Statement {
            ty: StatementType::BuiltinFunctionCall(BuiltinFunctionCall {
                kind: BuiltinFunctionKind::Codesize,
                args: vec![Argument {
                    arg_type: None,
                    name: Some("TEST".to_string()),
                    indexed: false,
                    arg_location: None,
                    span: AstSpan(vec![Span { start: 77, end: 80, file: None }]),
                }],
                span: AstSpan(vec![
                    Span { start: 66, end: 75, file: None },
                    Span { start: 77, end: 80, file: None },
                ]),
            }),
            span: AstSpan(vec![
                Span { start: 66, end: 75, file: None },
                Span { start: 77, end: 80, file: None },
            ]),
        }],
        takes: 0,
        returns: 0,
        span: AstSpan(vec![
            Span { start: 5, end: 11, file: None },
            Span { start: 13, end: 17, file: None },
            Span { start: 19, end: 30, file: None },
            Span { start: 31, end: 31, file: None },
            Span { start: 32, end: 32, file: None },
            Span { start: 34, end: 34, file: None },
            Span { start: 36, end: 40, file: None },
            Span { start: 41, end: 41, file: None },
            Span { start: 42, end: 42, file: None },
            Span { start: 43, end: 43, file: None },
            Span { start: 45, end: 51, file: None },
            Span { start: 52, end: 52, file: None },
            Span { start: 53, end: 53, file: None },
            Span { start: 54, end: 54, file: None },
            Span { start: 56, end: 56, file: None },
            Span { start: 66, end: 75, file: None },
            Span { start: 76, end: 76, file: None },
            Span { start: 77, end: 80, file: None },
            Span { start: 81, end: 81, file: None },
            Span { start: 87, end: 87, file: None },
        ]),
        outlined: false,
        test: false,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

// Outlined macros (functions) are parsed the same way as inlined macros, so there should be no
// difference besides the spans as well as the outlined flag.
#[test]
fn empty_outlined_macro() {
    let source = "#define fn HELLO_WORLD() = takes(0) returns(4) {}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);

    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        decorator: None,
        parameters: vec![],
        statements: vec![],
        takes: 0,
        returns: 4,
        span: AstSpan(vec![
            Span { start: 0, end: 6, file: None },
            Span { start: 8, end: 9, file: None },
            Span { start: 11, end: 21, file: None },
            Span { start: 22, end: 22, file: None },
            Span { start: 23, end: 23, file: None },
            Span { start: 25, end: 25, file: None },
            Span { start: 27, end: 31, file: None },
            Span { start: 32, end: 32, file: None },
            Span { start: 33, end: 33, file: None },
            Span { start: 34, end: 34, file: None },
            Span { start: 36, end: 42, file: None },
            Span { start: 43, end: 43, file: None },
            Span { start: 44, end: 44, file: None },
            Span { start: 45, end: 45, file: None },
            Span { start: 47, end: 47, file: None },
            Span { start: 48, end: 48, file: None },
        ]),
        outlined: true,
        test: false,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn outlined_macro_with_simple_body() {
    let source = "#define fn HELLO_WORLD() = takes(3) returns(0) {\n0x00 mstore\n 0x01 0x02 add\n}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        decorator: None,
        parameters: vec![],
        statements: vec![
            Statement {
                ty: StatementType::Literal(str_to_bytes32("00")),
                span: AstSpan(vec![Span { start: 51, end: 52, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Mstore),
                span: AstSpan(vec![Span { start: 54, end: 59, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("01")),
                span: AstSpan(vec![Span { start: 64, end: 65, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("02")),
                span: AstSpan(vec![Span { start: 69, end: 70, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Add),
                span: AstSpan(vec![Span { start: 72, end: 74, file: None }]),
            },
        ],
        takes: 3,
        returns: 0,
        span: AstSpan(vec![
            Span { start: 0, end: 6, file: None },
            Span { start: 8, end: 9, file: None },
            Span { start: 11, end: 21, file: None },
            Span { start: 22, end: 22, file: None },
            Span { start: 23, end: 23, file: None },
            Span { start: 25, end: 25, file: None },
            Span { start: 27, end: 31, file: None },
            Span { start: 32, end: 32, file: None },
            Span { start: 33, end: 33, file: None },
            Span { start: 34, end: 34, file: None },
            Span { start: 36, end: 42, file: None },
            Span { start: 43, end: 43, file: None },
            Span { start: 44, end: 44, file: None },
            Span { start: 45, end: 45, file: None },
            Span { start: 47, end: 47, file: None },
            Span { start: 51, end: 52, file: None },
            Span { start: 54, end: 59, file: None },
            Span { start: 64, end: 65, file: None },
            Span { start: 69, end: 70, file: None },
            Span { start: 72, end: 74, file: None },
            Span { start: 76, end: 76, file: None },
        ]),
        outlined: true,
        test: false,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn empty_test() {
    let source = "#define test HELLO_WORLD() = takes(0) returns(4) {}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        decorator: None,
        parameters: vec![],
        statements: vec![],
        takes: 0,
        returns: 4,
        span: AstSpan(vec![
            Span { start: 0, end: 6, file: None },
            Span { start: 8, end: 11, file: None },
            Span { start: 13, end: 23, file: None },
            Span { start: 24, end: 24, file: None },
            Span { start: 25, end: 25, file: None },
            Span { start: 27, end: 27, file: None },
            Span { start: 29, end: 33, file: None },
            Span { start: 34, end: 34, file: None },
            Span { start: 35, end: 35, file: None },
            Span { start: 36, end: 36, file: None },
            Span { start: 38, end: 44, file: None },
            Span { start: 45, end: 45, file: None },
            Span { start: 46, end: 46, file: None },
            Span { start: 47, end: 47, file: None },
            Span { start: 49, end: 49, file: None },
            Span { start: 50, end: 50, file: None },
        ]),
        outlined: false,
        test: true,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn test_with_simple_body() {
    let source =
        "#define test HELLO_WORLD() = takes(3) returns(0) {\n0x00 0x00 mstore\n 0x01 0x02 add\n}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        decorator: None,
        parameters: vec![],
        statements: vec![
            Statement {
                ty: StatementType::Literal([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
                span: AstSpan(vec![Span { start: 53, end: 54, file: None }]),
            },
            Statement {
                ty: StatementType::Literal([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
                span: AstSpan(vec![Span { start: 58, end: 59, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Mstore),
                span: AstSpan(vec![Span { start: 61, end: 66, file: None }]),
            },
            Statement {
                ty: StatementType::Literal([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ]),
                span: AstSpan(vec![Span { start: 71, end: 72, file: None }]),
            },
            Statement {
                ty: StatementType::Literal([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 2,
                ]),
                span: AstSpan(vec![Span { start: 76, end: 77, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Add),
                span: AstSpan(vec![Span { start: 79, end: 81, file: None }]),
            },
        ],
        takes: 3,
        returns: 0,
        span: AstSpan(vec![
            Span { start: 0, end: 6, file: None },
            Span { start: 8, end: 11, file: None },
            Span { start: 13, end: 23, file: None },
            Span { start: 24, end: 24, file: None },
            Span { start: 25, end: 25, file: None },
            Span { start: 27, end: 27, file: None },
            Span { start: 29, end: 33, file: None },
            Span { start: 34, end: 34, file: None },
            Span { start: 35, end: 35, file: None },
            Span { start: 36, end: 36, file: None },
            Span { start: 38, end: 44, file: None },
            Span { start: 45, end: 45, file: None },
            Span { start: 46, end: 46, file: None },
            Span { start: 47, end: 47, file: None },
            Span { start: 49, end: 49, file: None },
            Span { start: 53, end: 54, file: None },
            Span { start: 58, end: 59, file: None },
            Span { start: 61, end: 66, file: None },
            Span { start: 71, end: 72, file: None },
            Span { start: 76, end: 77, file: None },
            Span { start: 79, end: 81, file: None },
            Span { start: 83, end: 83, file: None },
        ]),
        outlined: false,
        test: true,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn empty_test_with_decorator() {
    let source = r#"
    #[value(0x01)]
    #define test MY_TEST() = takes(0) returns(0) {}
    "#;
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: String::from("MY_TEST"),
        decorator: Some(Decorator {
            flags: vec![DecoratorFlag::Value([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ])],
        }),
        parameters: vec![],
        statements: vec![],
        takes: 0,
        returns: 0,
        span: AstSpan(vec![
            Span { start: 5, end: 5, file: None },
            Span { start: 6, end: 6, file: None },
            Span { start: 7, end: 11, file: None },
            Span { start: 12, end: 12, file: None },
            Span { start: 15, end: 16, file: None },
            Span { start: 17, end: 17, file: None },
            Span { start: 18, end: 18, file: None },
            Span { start: 24, end: 30, file: None },
            Span { start: 32, end: 35, file: None },
            Span { start: 37, end: 43, file: None },
            Span { start: 44, end: 44, file: None },
            Span { start: 45, end: 45, file: None },
            Span { start: 47, end: 47, file: None },
            Span { start: 49, end: 53, file: None },
            Span { start: 54, end: 54, file: None },
            Span { start: 55, end: 55, file: None },
            Span { start: 56, end: 56, file: None },
            Span { start: 58, end: 64, file: None },
            Span { start: 65, end: 65, file: None },
            Span { start: 66, end: 66, file: None },
            Span { start: 67, end: 67, file: None },
            Span { start: 69, end: 69, file: None },
            Span { start: 70, end: 70, file: None },
        ]),
        outlined: false,
        test: true,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn empty_test_with_multi_flag_decorator() {
    let source = r#"
    #[calldata("0x02"), value(0x01)]
    #define test MY_TEST() = takes(0) returns(0) {}
    "#;
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: String::from("MY_TEST"),
        decorator: Some(Decorator {
            flags: vec![
                DecoratorFlag::Calldata(String::from("0x02")),
                DecoratorFlag::Value([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 1,
                ]),
            ],
        }),
        parameters: vec![],
        statements: vec![],
        takes: 0,
        returns: 0,
        span: AstSpan(vec![
            Span { start: 5, end: 5, file: None },
            Span { start: 6, end: 6, file: None },
            Span { start: 7, end: 14, file: None },
            Span { start: 15, end: 15, file: None },
            Span { start: 16, end: 21, file: None },
            Span { start: 22, end: 22, file: None },
            Span { start: 23, end: 23, file: None },
            Span { start: 25, end: 29, file: None },
            Span { start: 30, end: 30, file: None },
            Span { start: 33, end: 34, file: None },
            Span { start: 35, end: 35, file: None },
            Span { start: 36, end: 36, file: None },
            Span { start: 42, end: 48, file: None },
            Span { start: 50, end: 53, file: None },
            Span { start: 55, end: 61, file: None },
            Span { start: 62, end: 62, file: None },
            Span { start: 63, end: 63, file: None },
            Span { start: 65, end: 65, file: None },
            Span { start: 67, end: 71, file: None },
            Span { start: 72, end: 72, file: None },
            Span { start: 73, end: 73, file: None },
            Span { start: 74, end: 74, file: None },
            Span { start: 76, end: 82, file: None },
            Span { start: 83, end: 83, file: None },
            Span { start: 84, end: 84, file: None },
            Span { start: 85, end: 85, file: None },
            Span { start: 87, end: 87, file: None },
            Span { start: 88, end: 88, file: None },
        ]),
        outlined: false,
        test: true,
    };
    assert_eq!(macro_definition, expected);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}
