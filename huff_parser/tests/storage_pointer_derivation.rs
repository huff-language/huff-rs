use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn derives_storage_pointers() {
    let source =
        "#define constant FSP_LOCATION = FREE_STORAGE_POINTER()\n#define constant FSP_LOCATION_2 = FREE_STORAGE_POINTER()\n#define constant NUM = 0xa57B";

    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let mut contract = parser.parse().unwrap();
    assert_eq!(parser.current_token.kind, TokenKind::Eof);

    // Ensure that the constant definitions were parsed correctly
    let fsp_constant = contract.constants.lock().unwrap()[0].clone();
    assert_eq!(
        fsp_constant,
        ConstantDefinition {
            name: "FSP_LOCATION".to_string(),
            value: ConstVal::FreeStoragePointer(FreeStoragePointer {}),
            span: AstSpan(vec![
                Span { start: 0, end: 7, file: None },
                Span { start: 8, end: 16, file: None },
                Span { start: 17, end: 29, file: None },
                Span { start: 30, end: 31, file: None },
                Span { start: 32, end: 54, file: None }
            ])
        }
    );

    let fsp_constant = contract.constants.lock().unwrap()[1].clone();
    assert_eq!(
        fsp_constant,
        ConstantDefinition {
            name: "FSP_LOCATION_2".to_string(),
            value: ConstVal::FreeStoragePointer(FreeStoragePointer {}),
            span: AstSpan(vec![
                Span { start: 55, end: 62, file: None },
                Span { start: 63, end: 71, file: None },
                Span { start: 72, end: 86, file: None },
                Span { start: 87, end: 88, file: None },
                Span { start: 89, end: 111, file: None }
            ])
        }
    );

    let num_constant = contract.constants.lock().unwrap()[2].clone();
    assert_eq!(
        num_constant,
        ConstantDefinition {
            name: "NUM".to_string(),
            value: ConstVal::Literal(str_to_bytes32("a57B")),
            span: AstSpan(vec![
                Span { start: 112, end: 119, file: None },
                Span { start: 120, end: 128, file: None },
                Span { start: 129, end: 132, file: None },
                Span { start: 133, end: 134, file: None },
                Span { start: 137, end: 141, file: None }
            ])
        }
    );

    // Derive the AST's free storage pointers
    contract.derive_storage_pointers();

    // Ensure that the storage pointers were set for the FSP constants in the AST
    assert_eq!(
        contract.constants.lock().unwrap()[0].value,
        ConstVal::FreeStoragePointer(FreeStoragePointer)
    );
    assert_eq!(
        contract.constants.lock().unwrap()[1].value,
        ConstVal::FreeStoragePointer(FreeStoragePointer)
    );
    assert_eq!(
        contract.constants.lock().unwrap()[2].value,
        ConstVal::Literal(str_to_bytes32("a57B"))
    );
}
