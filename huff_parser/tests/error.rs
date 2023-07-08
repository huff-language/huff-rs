use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn test_parses_custom_error() {
    let source = "#define error TestError(uint256)";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
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
                span: AstSpan(vec![Span { start: 24, end: 30, file: None }]),
                arg_location: None,
            }],
            span: AstSpan(vec![
                Span { start: 0, end: 6, file: None },
                Span { start: 8, end: 12, file: None },
                Span { start: 14, end: 22, file: None },
                Span { start: 23, end: 23, file: None },
                Span { start: 24, end: 30, file: None },
                Span { start: 31, end: 31, file: None }
            ])
        }
    );
}

#[test]
fn test_error_sel_no_param() {
    let source = "#define error NotOwner()";

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let contract = parser.parse().unwrap();

    assert_eq!(parser.current_token.kind, TokenKind::Eof);

    let custom_error = contract.errors[0].clone();
    assert_eq!(
        custom_error,
        ErrorDefinition {
            name: String::from("NotOwner"),
            selector: [48, 205, 116, 113],
            parameters: vec![],
            span: AstSpan(vec![
                Span { start: 0, end: 6, file: None },
                Span { start: 8, end: 12, file: None },
                Span { start: 14, end: 21, file: None },
                Span { start: 22, end: 22, file: None },
                Span { start: 23, end: 23, file: None }
            ])
        }
    );
}
