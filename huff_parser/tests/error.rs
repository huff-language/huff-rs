use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn test_parses_custom_error() {
    let source = "#define error TestError(uint256)";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let contract = parser.parse().unwrap();
    assert_eq!(parser.current_token.kind, TokenKind::Eof);

    let custom_error = contract.errors[0].clone();
    assert_eq!(
        custom_error,
        ErrorDefinition {
            name: String::from("TestError"),
            selector: [124, 104, 44, 83],
            parameters: vec![Argument {
                arg_type: Some(String::from("uint256")),
                name: None,
                indexed: false,
                span: AstSpan(vec![Span { start: 24, end: 31, file: None }])
            }],
            span: AstSpan(vec![
                Span { start: 0, end: 7, file: None },
                Span { start: 8, end: 13, file: None },
                Span { start: 14, end: 23, file: None },
                Span { start: 23, end: 24, file: None },
                Span { start: 24, end: 31, file: None },
                Span { start: 31, end: 32, file: None }
            ])
        }
    );
}
