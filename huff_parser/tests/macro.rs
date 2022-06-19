use huff_lexer::*;
use huff_parser::*;
use huff_utils::{evm::Opcode, prelude::*};

#[test]
fn empty_macro() {
    let source = "#define macro HELLO_WORLD() = takes(0) returns(4) {}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        parameters: vec![],
        statements: vec![],
        takes: 0,
        returns: 4,
        span: AstSpan(vec![]),
    };
    assert_eq!(macro_definition.name, expected.name);
    assert_eq!(macro_definition.parameters, expected.parameters);
    assert_eq!(macro_definition.statements, expected.statements);
    assert_eq!(macro_definition.takes, expected.takes);
    assert_eq!(macro_definition.returns, expected.returns);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn macro_with_simple_body() {
    let source =
        "#define macro HELLO_WORLD() = takes(3) returns(0) {\n0x00 mstore\n 0x01 0x02 add\n}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        parameters: vec![],
        statements: vec![
            Statement {
                ty: StatementType::Literal(str_to_bytes32("00")),
                span: AstSpan(vec![
                    Span { start: 46, end: 47, file: None },
                    Span { start: 47, end: 48, file: None },
                    Span { start: 48, end: 49, file: None },
                    Span { start: 50, end: 51, file: None },
                ]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Mstore),
                span: AstSpan(vec![Span { start: 54, end: 56, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("01")),
                span: AstSpan(vec![Span { start: 57, end: 63, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("02")),
                span: AstSpan(vec![Span { start: 67, end: 69, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Add),
                span: AstSpan(vec![Span { start: 72, end: 74, file: None }]),
            },
        ],
        takes: 3,
        returns: 0,
        span: AstSpan(vec![]),
    };
    assert_eq!(macro_definition.name, expected.name);
    assert_eq!(macro_definition.parameters, expected.parameters);
    assert_eq!(macro_definition.statements, expected.statements);
    assert_eq!(macro_definition.takes, expected.takes);
    assert_eq!(macro_definition.returns, expected.returns);
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
    let lexer = Lexer::new(flattened_source);
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
        }],
        statements: vec![
            Statement {
                ty: StatementType::Opcode(Opcode::Dup2),
                span: AstSpan(vec![
                    Span { start: 93, end: 94, file: None },
                    Span { start: 94, end: 95, file: None },
                    Span { start: 95, end: 96, file: None },
                    Span { start: 97, end: 98, file: None },
                ]),
            },
            Statement {
                ty: StatementType::Constant("BALANCE_LOCATION".to_string()),
                span: AstSpan(vec![Span { start: 209, end: 213, file: None }]),
            },
            Statement {
                ty: StatementType::MacroInvocation(MacroInvocation {
                    macro_name: "LOAD_ELEMENT_FROM_KEYS".to_string(),
                    args: vec![MacroArg::Literal(str_to_bytes32("00"))],
                }),
                span: AstSpan(vec![
                    Span { start: 264, end: 265, file: None },
                    Span { start: 265, end: 281, file: None },
                    Span { start: 281, end: 282, file: None },
                ]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Dup1),
                span: AstSpan(vec![
                    Span { start: 283, end: 305, file: None },
                    Span { start: 305, end: 306, file: None },
                    Span { start: 308, end: 310, file: None },
                    Span { start: 310, end: 311, file: None },
                ]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Dup3),
                span: AstSpan(vec![Span { start: 351, end: 355, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Gt),
                span: AstSpan(vec![Span { start: 418, end: 422, file: None }]),
            },
            Statement {
                ty: StatementType::ArgCall("error".to_string()),
                span: AstSpan(vec![Span { start: 492, end: 494, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Jumpi),
                span: AstSpan(vec![
                    Span { start: 565, end: 566, file: None },
                    Span { start: 566, end: 571, file: None },
                    Span { start: 571, end: 572, file: None },
                ]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Dup2),
                span: AstSpan(vec![Span { start: 573, end: 578, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Swap1),
                span: AstSpan(vec![Span { start: 715, end: 719, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Sub),
                span: AstSpan(vec![Span { start: 780, end: 785, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Dup3),
                span: AstSpan(vec![Span { start: 845, end: 848, file: None }]),
            },
            Statement {
                ty: StatementType::Constant("BALANCE_LOCATION".to_string()),
                span: AstSpan(vec![Span { start: 911, end: 915, file: None }]),
            },
            Statement {
                ty: StatementType::MacroInvocation(MacroInvocation {
                    macro_name: "STORE_ELEMENT_FROM_KEYS".to_string(),
                    args: vec![MacroArg::Literal(str_to_bytes32("00"))],
                }),
                span: AstSpan(vec![
                    Span { start: 981, end: 982, file: None },
                    Span { start: 982, end: 998, file: None },
                    Span { start: 998, end: 999, file: None },
                ]),
            },
        ],
        takes: 3,
        returns: 3,
        span: AstSpan(vec![]),
    };
    assert_eq!(macro_definition.name, expected.name);
    assert_eq!(macro_definition.parameters, expected.parameters);
    assert_eq!(macro_definition.statements, expected.statements);
    assert_eq!(macro_definition.takes, expected.takes);
    assert_eq!(macro_definition.returns, expected.returns);
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
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "LABEL_FILLED".to_string(),
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
                            }),
                            span: AstSpan(vec![
                                Span { start: 66, end: 75, file: None },
                                Span { start: 75, end: 76, file: None },
                            ]),
                        },
                        Statement {
                            ty: StatementType::Literal(str_to_bytes32("00")),
                            span: AstSpan(vec![
                                Span { start: 89, end: 105, file: None },
                                Span { start: 105, end: 106, file: None },
                                Span { start: 106, end: 107, file: None },
                            ]),
                        },
                        Statement {
                            ty: StatementType::Literal(str_to_bytes32("00")),
                            span: AstSpan(vec![Span { start: 122, end: 124, file: None }]),
                        },
                        Statement {
                            ty: StatementType::Opcode(Opcode::Revert),
                            span: AstSpan(vec![Span { start: 127, end: 129, file: None }]),
                        },
                    ],
                }),
                span: AstSpan(vec![
                    Span { start: 52, end: 53, file: None },
                    Span { start: 53, end: 54, file: None },
                    Span { start: 54, end: 55, file: None },
                    Span { start: 56, end: 57, file: None },
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
                            }),
                            span: AstSpan(vec![
                                Span { start: 145, end: 150, file: None },
                                Span { start: 150, end: 151, file: None },
                            ]),
                        },
                        Statement {
                            ty: StatementType::Literal(str_to_bytes32("00")),
                            span: AstSpan(vec![
                                Span { start: 164, end: 180, file: None },
                                Span { start: 180, end: 181, file: None },
                                Span { start: 181, end: 182, file: None },
                            ]),
                        },
                        Statement {
                            ty: StatementType::Literal(str_to_bytes32("00")),
                            span: AstSpan(vec![Span { start: 197, end: 199, file: None }]),
                        },
                        Statement {
                            ty: StatementType::Opcode(Opcode::Revert),
                            span: AstSpan(vec![Span { start: 202, end: 204, file: None }]),
                        },
                    ],
                }),
                span: AstSpan(vec![Span { start: 130, end: 136, file: None }]),
            },
        ],
        takes: 0,
        returns: 0,
        span: AstSpan(vec![]),
    };
    assert_eq!(macro_definition.name, expected.name);
    assert_eq!(macro_definition.parameters, expected.parameters);
    assert_eq!(macro_definition.statements, expected.statements);
    assert_eq!(macro_definition.takes, expected.takes);
    assert_eq!(macro_definition.returns, expected.returns);
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
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "ARG_CALL".to_string(),
        parameters: vec![Argument {
            arg_type: None,
            name: Some("error".to_string()),
            indexed: false,
        }],
        statements: vec![
            Statement {
                ty: StatementType::MacroInvocation(MacroInvocation {
                    macro_name: "TRANSFER_TAKE_FROM".to_string(),
                    args: vec![MacroArg::ArgCall("error".to_string())],
                }),
                span: AstSpan(vec![
                    Span { start: 53, end: 54, file: None },
                    Span { start: 54, end: 55, file: None },
                    Span { start: 55, end: 56, file: None },
                    Span { start: 57, end: 58, file: None },
                ]),
            },
            Statement {
                ty: StatementType::MacroInvocation(MacroInvocation {
                    macro_name: "TRANSFER_GIVE_TO".to_string(),
                    args: vec![MacroArg::ArgCall("error".to_string())],
                }),
                span: AstSpan(vec![
                    Span { start: 67, end: 85, file: None },
                    Span { start: 85, end: 86, file: None },
                    Span { start: 86, end: 87, file: None },
                    Span { start: 87, end: 92, file: None },
                    Span { start: 92, end: 93, file: None },
                    Span { start: 93, end: 94, file: None },
                ]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("01")),
                span: AstSpan(vec![
                    Span { start: 103, end: 119, file: None },
                    Span { start: 119, end: 120, file: None },
                    Span { start: 120, end: 121, file: None },
                    Span { start: 121, end: 126, file: None },
                    Span { start: 126, end: 127, file: None },
                    Span { start: 127, end: 128, file: None },
                ]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("00")),
                span: AstSpan(vec![Span { start: 140, end: 142, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Mstore),
                span: AstSpan(vec![Span { start: 145, end: 147, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("20")),
                span: AstSpan(vec![Span { start: 148, end: 154, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("00")),
                span: AstSpan(vec![Span { start: 165, end: 167, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Return),
                span: AstSpan(vec![Span { start: 170, end: 172, file: None }]),
            },
        ],
        takes: 0,
        returns: 0,
        span: AstSpan(vec![]),
    };
    assert_eq!(macro_definition.name, expected.name);
    assert_eq!(macro_definition.parameters, expected.parameters);
    assert_eq!(macro_definition.statements, expected.statements);
    assert_eq!(macro_definition.takes, expected.takes);
    assert_eq!(macro_definition.returns, expected.returns);
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
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let expected = MacroDefinition {
        name: "BUILTIN_TEST".to_string(),
        parameters: vec![],
        statements: vec![Statement {
            ty: StatementType::BuiltinFunctionCall(BuiltinFunctionCall {
                kind: BuiltinFunctionKind::Codesize,
                args: vec![Argument {
                    arg_type: None,
                    name: Some("TEST".to_string()),
                    indexed: false,
                }],
            }),
            span: AstSpan(vec![
                Span { start: 52, end: 53, file: None },
                Span { start: 53, end: 54, file: None },
                Span { start: 54, end: 55, file: None },
                Span { start: 56, end: 57, file: None },
            ]),
        }],
        takes: 0,
        returns: 0,
        span: AstSpan(vec![]),
    };
    assert_eq!(macro_definition.name, expected.name);
    assert_eq!(macro_definition.parameters, expected.parameters);
    assert_eq!(macro_definition.statements, expected.statements);
    assert_eq!(macro_definition.takes, expected.takes);
    assert_eq!(macro_definition.returns, expected.returns);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}
