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
            Statement::Literal(str_to_bytes32("00")),
            Statement::Opcode(Opcode::Mstore),
            Statement::Literal(str_to_bytes32("01")),
            Statement::Literal(str_to_bytes32("02")),
            Statement::Opcode(Opcode::Add),
            Statement::Label(Label {
                name: "cool_label".to_string(),
                inner: vec![
                    Statement::MacroInvocation(MacroInvocation {
                        macro_name: "HELLO".to_string(),
                        args: vec![],
                    }),
                    Statement::Literal(str_to_bytes32("00")),
                    Statement::Literal(str_to_bytes32("00")),
                    Statement::Opcode(Opcode::Revert),
                ],
            }),
        ],
        takes: 3,
        returns: 0,
        span: AstSpan(vec![]),
    };
    assert_eq!(macro_definition.name, md_expected.name);
    assert_eq!(macro_definition.parameters, md_expected.parameters);
    assert_eq!(macro_definition.statements, md_expected.statements);
    assert_eq!(macro_definition.takes, md_expected.takes);
    assert_eq!(macro_definition.returns, md_expected.returns);
    // TODO: Test Macro Definition Span
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}
