use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn parses_free_storage_pointer_constant() {
    let c = "#define constant FSP_LOCATION = FREE_STORAGE_POINTER()";

    let lexer = Lexer::new(c);
    let tokens = lexer.into_iter().collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    let contract = parser.parse().unwrap();

    let fsp_constant = contract.constants[0].clone();
    assert_eq!(
        fsp_constant,
        ConstantDefinition {
            name: "FSP_LOCATION".to_string(),
            value: ConstVal::FreeStoragePointer(FreeStoragePointer {})
        }
    );
}

#[test]
fn parses_literal_constant() {
    let c = "#define constant LITERAL = 0x8C5BE1E5EBEC7D5BD14F71427D1E84F3DD0314C0F7B2291E5B200AC8C7C3B925";

    let lexer = Lexer::new(c);
    let tokens = lexer.into_iter().collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    let contract = parser.parse().unwrap();

    // Create const val
    let arr: [u8; 32] =
        str_to_bytes32("8C5BE1E5EBEC7D5BD14F71427D1E84F3DD0314C0F7B2291E5B200AC8C7C3B925");

    // Check Literal
    let fsp_constant = contract.constants[0].clone();
    assert_eq!(
        fsp_constant,
        ConstantDefinition { name: "LITERAL".to_string(), value: ConstVal::Literal(arr) }
    );
}
