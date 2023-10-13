use huff_lexer::Lexer;
use huff_parser::*;
use huff_utils::{evm::Opcode, prelude::*};

#[test]
fn macro_with_simple_body() {
    let source =
        "#define macro HELLO_WORLD() = takes(3) returns(0) {\n #define padded(17) {\n 0x00 mstore\n 0x01 0x02 add \n} 0x69 0x69 return\n}";
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
