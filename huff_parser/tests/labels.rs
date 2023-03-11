use huff_lexer::*;
use huff_parser::*;
use huff_utils::{evm::Opcode, prelude::*};

#[test]
fn multiline_labels() {
    let source = r#"
    #define macro HELLO_WORLD() = takes(3) returns(0) {
      0x00 mstore
      0x01 0x02 add
      cool_label:
        HELLO()
        0x00 0x00 revert
    }
    "#;
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let md_expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        decorator: None,
        parameters: vec![],
        statements: vec![
            Statement {
                ty: StatementType::Literal(str_to_bytes32("00")),
                span: AstSpan(vec![Span { start: 65, end: 66, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Mstore),
                span: AstSpan(vec![Span { start: 68, end: 73, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("01")),
                span: AstSpan(vec![Span { start: 83, end: 84, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("02")),
                span: AstSpan(vec![Span { start: 88, end: 89, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Add),
                span: AstSpan(vec![Span { start: 91, end: 93, file: None }]),
            },
            Statement {
                ty: StatementType::Label(Label {
                    name: "cool_label".to_string(),
                    inner: vec![
                        Statement {
                            ty: StatementType::MacroInvocation(MacroInvocation {
                                macro_name: "HELLO".to_string(),
                                args: vec![],
                                span: AstSpan(vec![
                                    Span { start: 121, end: 125, file: None },
                                    Span { start: 126, end: 126, file: None },
                                    Span { start: 127, end: 127, file: None },
                                ]),
                            }),
                            span: AstSpan(vec![
                                Span { start: 121, end: 125, file: None },
                                Span { start: 126, end: 126, file: None },
                                Span { start: 127, end: 127, file: None },
                            ]),
                        },
                        Statement {
                            ty: StatementType::Literal(str_to_bytes32("00")),
                            span: AstSpan(vec![Span { start: 139, end: 140, file: None }]),
                        },
                        Statement {
                            ty: StatementType::Literal(str_to_bytes32("00")),
                            span: AstSpan(vec![Span { start: 144, end: 145, file: None }]),
                        },
                        Statement {
                            ty: StatementType::Opcode(Opcode::Revert),
                            span: AstSpan(vec![Span { start: 147, end: 152, file: None }]),
                        },
                    ],
                    span: AstSpan(vec![
                        Span { start: 101, end: 110, file: None },
                        Span { start: 121, end: 125, file: None },
                        Span { start: 126, end: 126, file: None },
                        Span { start: 127, end: 127, file: None },
                        Span { start: 139, end: 140, file: None },
                        Span { start: 144, end: 145, file: None },
                        Span { start: 147, end: 152, file: None },
                    ]),
                }),
                span: AstSpan(vec![
                    Span { start: 101, end: 110, file: None },
                    Span { start: 121, end: 125, file: None },
                    Span { start: 126, end: 126, file: None },
                    Span { start: 127, end: 127, file: None },
                    Span { start: 139, end: 140, file: None },
                    Span { start: 144, end: 145, file: None },
                    Span { start: 147, end: 152, file: None },
                ]),
            },
        ],
        takes: 3,
        returns: 0,
        span: AstSpan(vec![
            // "#define"
            Span { start: 5, end: 11, file: None },
            // "macro"
            Span { start: 13, end: 17, file: None },
            // "HELLO_WORLD"
            Span { start: 19, end: 29, file: None },
            // "("
            Span { start: 30, end: 30, file: None },
            // ")"
            Span { start: 31, end: 31, file: None },
            // "="
            Span { start: 33, end: 33, file: None },
            // "takes"
            Span { start: 35, end: 39, file: None },
            // "("
            Span { start: 40, end: 40, file: None },
            // "0"
            Span { start: 41, end: 41, file: None },
            // ")"
            Span { start: 42, end: 42, file: None },
            // "returns"
            Span { start: 44, end: 50, file: None },
            // "("
            Span { start: 51, end: 51, file: None },
            // "0"
            Span { start: 52, end: 52, file: None },
            // ")"
            Span { start: 53, end: 53, file: None },
            // "{"
            Span { start: 55, end: 55, file: None },
            // "0x00"
            Span { start: 65, end: 66, file: None },
            // "mstore"
            Span { start: 68, end: 73, file: None },
            // "0x01"
            Span { start: 83, end: 84, file: None },
            // "0x02"
            Span { start: 88, end: 89, file: None },
            // "add"
            Span { start: 91, end: 93, file: None },
            // "cool_label"
            Span { start: 101, end: 110, file: None },
            // ":"
            Span { start: 111, end: 111, file: None },
            // "HELLO"
            Span { start: 121, end: 125, file: None },
            // "("
            Span { start: 126, end: 126, file: None },
            // ")"
            Span { start: 127, end: 127, file: None },
            // "0x00"
            Span { start: 139, end: 140, file: None },
            // "0x00"
            Span { start: 144, end: 145, file: None },
            // "revert"
            Span { start: 147, end: 152, file: None },
            // "}"
            Span { start: 158, end: 158, file: None },
        ]),
        outlined: false,
        test: false,
    };
    assert_eq!(macro_definition.name, md_expected.name);
    assert_eq!(macro_definition.parameters, md_expected.parameters);
    assert_eq!(macro_definition.takes, md_expected.takes);
    assert_eq!(macro_definition.returns, md_expected.returns);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
    assert_eq!(macro_definition.span, md_expected.span);

    // Test that each statement is the correct type
    for (i, s) in macro_definition.statements.iter().enumerate() {
        assert_eq!(s.ty, md_expected.statements[i].ty);
        assert_eq!(s.span, md_expected.statements[i].span);
    }
}

#[test]
pub fn builtins_under_labels() {
    let source = r#"
    #define function myFunc() pure returns (uint256)

    #define error TestError()

    #define jumptable__packed TEST_TABLE {
        my_label
    }

    #define macro SMALL_MACRO() = takes (3) returns (0) {
        0x20 0x00 mstore
    }

    #define macro HELLO_WORLD() = takes(3) returns(0) {
        my_label:
            __tablestart(TEST_TABLE)
            __tablesize(TEST_TABLE)
            __codesize(SMALL_MACRO)
            __FUNC_SIG(myFunc)
            __ERROR(TestError)
            __RIGHTPAD(0xBB)
    }
    "#;
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[1].clone();
    let md_expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        parameters: vec![],
        decorator: None,
        statements: vec![Statement {
            ty: StatementType::Label(Label {
                name: String::from("my_label"),
                inner: vec![
                    Statement {
                        ty: StatementType::BuiltinFunctionCall(BuiltinFunctionCall {
                            kind: BuiltinFunctionKind::Tablestart,
                            args: vec![Argument {
                                arg_type: None,
                                name: Some(String::from("TEST_TABLE")),
                                indexed: false,
                                arg_location: None,
                                span: AstSpan(vec![Span { start: 342, end: 351, file: None }]),
                            }],
                            span: AstSpan(vec![
                                Span { start: 329, end: 340, file: None },
                                Span { start: 342, end: 351, file: None },
                            ]),
                        }),
                        span: AstSpan(vec![
                            Span { start: 329, end: 340, file: None },
                            Span { start: 342, end: 351, file: None },
                        ]),
                    },
                    Statement {
                        ty: StatementType::BuiltinFunctionCall(BuiltinFunctionCall {
                            kind: BuiltinFunctionKind::Tablesize,
                            args: vec![Argument {
                                arg_type: None,
                                name: Some(String::from("TEST_TABLE")),
                                indexed: false,
                                arg_location: None,
                                span: AstSpan(vec![Span { start: 378, end: 387, file: None }]),
                            }],
                            span: AstSpan(vec![
                                Span { start: 366, end: 376, file: None },
                                Span { start: 378, end: 387, file: None },
                            ]),
                        }),
                        span: AstSpan(vec![
                            Span { start: 366, end: 376, file: None },
                            Span { start: 378, end: 387, file: None },
                        ]),
                    },
                    Statement {
                        ty: StatementType::BuiltinFunctionCall(BuiltinFunctionCall {
                            kind: BuiltinFunctionKind::Codesize,
                            args: vec![Argument {
                                arg_type: None,
                                name: Some(String::from("SMALL_MACRO")),
                                indexed: false,
                                arg_location: None,
                                span: AstSpan(vec![Span { start: 413, end: 423, file: None }]),
                            }],
                            span: AstSpan(vec![
                                Span { start: 402, end: 411, file: None },
                                Span { start: 413, end: 423, file: None },
                            ]),
                        }),
                        span: AstSpan(vec![
                            Span { start: 402, end: 411, file: None },
                            Span { start: 413, end: 423, file: None },
                        ]),
                    },
                    Statement {
                        ty: StatementType::BuiltinFunctionCall(BuiltinFunctionCall {
                            kind: BuiltinFunctionKind::FunctionSignature,
                            args: vec![Argument {
                                arg_type: None,
                                name: Some(String::from("myFunc")),
                                indexed: false,
                                arg_location: None,
                                span: AstSpan(vec![Span { start: 449, end: 454, file: None }]),
                            }],
                            span: AstSpan(vec![
                                Span { start: 438, end: 447, file: None },
                                Span { start: 449, end: 454, file: None },
                            ]),
                        }),
                        span: AstSpan(vec![
                            Span { start: 438, end: 447, file: None },
                            Span { start: 449, end: 454, file: None },
                        ]),
                    },
                    Statement {
                        ty: StatementType::BuiltinFunctionCall(BuiltinFunctionCall {
                            kind: BuiltinFunctionKind::Error,
                            args: vec![Argument {
                                arg_type: None,
                                name: Some(String::from("TestError")),
                                indexed: false,
                                arg_location: None,
                                span: AstSpan(vec![Span { start: 477, end: 485, file: None }]),
                            }],
                            span: AstSpan(vec![
                                Span { start: 469, end: 475, file: None },
                                Span { start: 477, end: 485, file: None },
                            ]),
                        }),
                        span: AstSpan(vec![
                            Span { start: 469, end: 475, file: None },
                            Span { start: 477, end: 485, file: None },
                        ]),
                    },
                    Statement {
                        ty: StatementType::BuiltinFunctionCall(BuiltinFunctionCall {
                            kind: BuiltinFunctionKind::RightPad,
                            args: vec![Argument {
                                arg_type: None,
                                name: Some(String::from("bb")),
                                indexed: false,
                                arg_location: None,
                                span: AstSpan(vec![Span { start: 513, end: 514, file: None }]),
                            }],
                            span: AstSpan(vec![
                                Span { start: 500, end: 509, file: None },
                                Span { start: 513, end: 514, file: None },
                            ]),
                        }),
                        span: AstSpan(vec![
                            Span { start: 500, end: 509, file: None },
                            Span { start: 513, end: 514, file: None },
                        ]),
                    },
                ],
                span: AstSpan(vec![
                    Span { start: 307, end: 314, file: None },
                    Span { start: 329, end: 340, file: None },
                    Span { start: 342, end: 351, file: None },
                    Span { start: 366, end: 376, file: None },
                    Span { start: 378, end: 387, file: None },
                    Span { start: 402, end: 411, file: None },
                    Span { start: 413, end: 423, file: None },
                    Span { start: 438, end: 447, file: None },
                    Span { start: 449, end: 454, file: None },
                    Span { start: 469, end: 475, file: None },
                    Span { start: 477, end: 485, file: None },
                    Span { start: 500, end: 509, file: None },
                    Span { start: 513, end: 514, file: None },
                ]),
            }),
            span: AstSpan(vec![
                Span { start: 307, end: 314, file: None },
                Span { start: 329, end: 340, file: None },
                Span { start: 342, end: 351, file: None },
                Span { start: 366, end: 376, file: None },
                Span { start: 378, end: 387, file: None },
                Span { start: 402, end: 411, file: None },
                Span { start: 413, end: 423, file: None },
                Span { start: 438, end: 447, file: None },
                Span { start: 449, end: 454, file: None },
                Span { start: 469, end: 475, file: None },
                Span { start: 477, end: 485, file: None },
                Span { start: 500, end: 509, file: None },
                Span { start: 513, end: 514, file: None },
            ]),
        }],
        takes: 3,
        returns: 0,
        span: AstSpan(vec![
            Span { start: 247, end: 253, file: None },
            Span { start: 255, end: 259, file: None },
            Span { start: 261, end: 271, file: None },
            Span { start: 272, end: 272, file: None },
            Span { start: 273, end: 273, file: None },
            Span { start: 275, end: 275, file: None },
            Span { start: 277, end: 281, file: None },
            Span { start: 282, end: 282, file: None },
            Span { start: 283, end: 283, file: None },
            Span { start: 284, end: 284, file: None },
            Span { start: 286, end: 292, file: None },
            Span { start: 293, end: 293, file: None },
            Span { start: 294, end: 294, file: None },
            Span { start: 295, end: 295, file: None },
            Span { start: 297, end: 297, file: None },
            Span { start: 307, end: 314, file: None },
            Span { start: 315, end: 315, file: None },
            Span { start: 329, end: 340, file: None },
            Span { start: 341, end: 341, file: None },
            Span { start: 342, end: 351, file: None },
            Span { start: 352, end: 352, file: None },
            Span { start: 366, end: 376, file: None },
            Span { start: 377, end: 377, file: None },
            Span { start: 378, end: 387, file: None },
            Span { start: 388, end: 388, file: None },
            Span { start: 402, end: 411, file: None },
            Span { start: 412, end: 412, file: None },
            Span { start: 413, end: 423, file: None },
            Span { start: 424, end: 424, file: None },
            Span { start: 438, end: 447, file: None },
            Span { start: 448, end: 448, file: None },
            Span { start: 449, end: 454, file: None },
            Span { start: 455, end: 455, file: None },
            Span { start: 469, end: 475, file: None },
            Span { start: 476, end: 476, file: None },
            Span { start: 477, end: 485, file: None },
            Span { start: 486, end: 486, file: None },
            Span { start: 500, end: 509, file: None },
            Span { start: 510, end: 510, file: None },
            Span { start: 513, end: 514, file: None },
            Span { start: 515, end: 515, file: None },
            Span { start: 521, end: 521, file: None },
        ]),
        outlined: false,
        test: false,
    };
    assert_eq!(macro_definition.name, md_expected.name);
    assert_eq!(macro_definition.parameters, md_expected.parameters);
    assert_eq!(macro_definition.takes, md_expected.takes);
    assert_eq!(macro_definition.returns, md_expected.returns);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
    assert_eq!(macro_definition.span, md_expected.span);

    // Test that each statement is the correct type
    for (i, s) in macro_definition.statements.iter().enumerate() {
        assert_eq!(s.ty, md_expected.statements[i].ty);
        assert_eq!(s.span, md_expected.statements[i].span);
    }
}
