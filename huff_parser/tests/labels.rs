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
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    let md_expected = MacroDefinition {
        name: "HELLO_WORLD".to_string(),
        parameters: vec![],
        statements: vec![
            Statement {
                ty: StatementType::Literal(str_to_bytes32("00")),
                span: AstSpan(vec![Span { start: 65, end: 67, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Mstore),
                span: AstSpan(vec![Span { start: 68, end: 74, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("01")),
                span: AstSpan(vec![Span { start: 83, end: 85, file: None }]),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("02")),
                span: AstSpan(vec![Span { start: 88, end: 90, file: None }]),
            },
            Statement {
                ty: StatementType::Opcode(Opcode::Add),
                span: AstSpan(vec![Span { start: 91, end: 94, file: None }]),
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
                                    Span { start: 121, end: 126, file: None },
                                    Span { start: 126, end: 127, file: None },
                                    Span { start: 127, end: 128, file: None },
                                ]),
                            }),
                            span: AstSpan(vec![
                                Span { start: 121, end: 126, file: None },
                                Span { start: 126, end: 127, file: None },
                                Span { start: 127, end: 128, file: None },
                            ]),
                        },
                        Statement {
                            ty: StatementType::Literal(str_to_bytes32("00")),
                            span: AstSpan(vec![Span { start: 139, end: 141, file: None }]),
                        },
                        Statement {
                            ty: StatementType::Literal(str_to_bytes32("00")),
                            span: AstSpan(vec![Span { start: 144, end: 146, file: None }]),
                        },
                        Statement {
                            ty: StatementType::Opcode(Opcode::Revert),
                            span: AstSpan(vec![Span { start: 147, end: 153, file: None }]),
                        },
                    ],
                    span: AstSpan(vec![
                        Span { start: 101, end: 111, file: None },
                        Span { start: 121, end: 126, file: None },
                        Span { start: 126, end: 127, file: None },
                        Span { start: 127, end: 128, file: None },
                        Span { start: 139, end: 141, file: None },
                        Span { start: 144, end: 146, file: None },
                        Span { start: 147, end: 153, file: None },
                    ]),
                }),
                span: AstSpan(vec![
                    Span { start: 101, end: 111, file: None },
                    Span { start: 121, end: 126, file: None },
                    Span { start: 126, end: 127, file: None },
                    Span { start: 127, end: 128, file: None },
                    Span { start: 139, end: 141, file: None },
                    Span { start: 144, end: 146, file: None },
                    Span { start: 147, end: 153, file: None },
                ]),
            },
        ],
        takes: 3,
        returns: 0,
        span: AstSpan(vec![
            // "#define"
            Span { start: 5, end: 12, file: None },
            // "macro"
            Span { start: 13, end: 18, file: None },
            // "HELLO_WORLD"
            Span { start: 19, end: 30, file: None },
            // "("
            Span { start: 30, end: 31, file: None },
            // ")"
            Span { start: 31, end: 32, file: None },
            // "="
            Span { start: 33, end: 34, file: None },
            // "takes"
            Span { start: 35, end: 40, file: None },
            // "("
            Span { start: 40, end: 41, file: None },
            // "0"
            Span { start: 41, end: 42, file: None },
            // ")"
            Span { start: 42, end: 43, file: None },
            // "returns"
            Span { start: 44, end: 51, file: None },
            // "("
            Span { start: 51, end: 52, file: None },
            // "0"
            Span { start: 52, end: 53, file: None },
            // ")"
            Span { start: 53, end: 54, file: None },
            // "{"
            Span { start: 55, end: 56, file: None },
            // "0x00"
            Span { start: 65, end: 67, file: None },
            // "mstore"
            Span { start: 68, end: 74, file: None },
            // "0x01"
            Span { start: 83, end: 85, file: None },
            // "0x02"
            Span { start: 88, end: 90, file: None },
            // "add"
            Span { start: 91, end: 94, file: None },
            // "cool_label"
            Span { start: 101, end: 111, file: None },
            // ":"
            Span { start: 111, end: 112, file: None },
            // "HELLO"
            Span { start: 121, end: 126, file: None },
            // "("
            Span { start: 126, end: 127, file: None },
            // ")"
            Span { start: 127, end: 128, file: None },
            // "0x00"
            Span { start: 139, end: 141, file: None },
            // "0x00"
            Span { start: 144, end: 146, file: None },
            // "revert"
            Span { start: 147, end: 153, file: None },
            // "}"
            Span { start: 158, end: 159, file: None },
        ]),
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
