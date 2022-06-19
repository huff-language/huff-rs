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
                span: AstSpan::default(),
            },
            Statement { ty: StatementType::Opcode(Opcode::Mstore), span: AstSpan::default() },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("01")),
                span: AstSpan::default(),
            },
            Statement {
                ty: StatementType::Literal(str_to_bytes32("02")),
                span: AstSpan::default(),
            },
            Statement { ty: StatementType::Opcode(Opcode::Add), span: AstSpan::default() },
            Statement {
                ty: StatementType::Label(Label {
                    name: "cool_label".to_string(),
                    inner: vec![
                        Statement {
                            ty: StatementType::MacroInvocation(MacroInvocation {
                                macro_name: "HELLO".to_string(),
                                args: vec![],
                                span: AstSpan::default(),
                            }),
                            span: AstSpan(vec![
                                Span { start: 101, end: 111, file: None },
                                Span { start: 111, end: 112, file: None },
                            ]),
                        },
                        Statement {
                            ty: StatementType::Literal(str_to_bytes32("00")),
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
                            ty: StatementType::Opcode(Opcode::Revert),
                            span: AstSpan(vec![Span { start: 144, end: 146, file: None }]),
                        },
                    ],
                    span: AstSpan::default(),
                }),
                span: AstSpan::default(),
            },
        ],
        takes: 3,
        returns: 0,
        span: AstSpan(vec![]),
    };
    assert_eq!(macro_definition.name, md_expected.name);
    assert_eq!(macro_definition.parameters, md_expected.parameters);
    assert_eq!(macro_definition.takes, md_expected.takes);
    assert_eq!(macro_definition.returns, md_expected.returns);
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
    // TODO: Test Macro Definition Span

    // Test that each statement is the correct type
    // TODO: Test each statement's span
    for (i, s) in macro_definition.statements.iter().enumerate() {
        assert_eq!(s.ty, md_expected.statements[i].ty);
    }
}
