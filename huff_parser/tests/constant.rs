use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn test_parses_free_storage_pointer_constant() {
    let source = "#define constant FSP_LOCATION = FREE_STORAGE_POINTER()";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let contract = parser.parse().unwrap();
    assert_eq!(parser.current_token.kind, TokenKind::Eof);

    let fsp_constant = contract.constants.lock().unwrap()[0].clone();
    assert_eq!(
        fsp_constant,
        ConstantDefinition {
            name: "FSP_LOCATION".to_string(),
            value: ConstVal::FreeStoragePointer(FreeStoragePointer {}),
            span: AstSpan(vec![
                Span { start: 0, end: 6, file: None },
                Span { start: 8, end: 15, file: None },
                Span { start: 17, end: 28, file: None },
                Span { start: 30, end: 30, file: None },
                Span { start: 32, end: 53, file: None }
            ])
        }
    );
}

#[test]
fn test_parses_literal_constant() {
    let source = "#define constant LITERAL = 0x8C5BE1E5EBEC7D5BD14F71427D1E84F3DD0314C0F7B2291E5B200AC8C7C3B925";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let contract = parser.parse().unwrap();
    assert_eq!(parser.current_token.kind, TokenKind::Eof);

    // Create const val
    let arr: [u8; 32] =
        str_to_bytes32("8C5BE1E5EBEC7D5BD14F71427D1E84F3DD0314C0F7B2291E5B200AC8C7C3B925");

    // Check Literal
    let fsp_constant = contract.constants.lock().unwrap()[0].clone();
    assert_eq!(
        fsp_constant,
        ConstantDefinition {
            name: "LITERAL".to_string(),
            value: ConstVal::Literal(arr),
            span: AstSpan(vec![
                Span { start: 0, end: 6, file: None },
                Span { start: 8, end: 15, file: None },
                Span { start: 17, end: 23, file: None },
                Span { start: 25, end: 25, file: None },
                Span { start: 29, end: 92, file: None }
            ])
        }
    );
}
