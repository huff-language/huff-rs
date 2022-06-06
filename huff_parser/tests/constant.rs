use bytes::BytesMut;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn parses_free_storage_pointer_constant() {
    let c = "#define constant FSP_LOCATION = FREE_STORAGE_POINTER()";

    let lexer = Lexer::new(c);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    let contract = parser.parse().unwrap();
    assert_eq!(parser.current_token.kind, TokenKind::Eof);

    let fsp_constant = contract.constants[0].clone();
    assert_eq!(
        fsp_constant,
        ConstantDefinition {
            name: "FSP_LOCATION",
            value: ConstVal::FreeStoragePointer(FreeStoragePointer {})
        }
    );
}

#[test]
fn parses_literal_constant() {
    let c = "#define constant LITERAL = 0x8C5BE1E5EBEC7D5BD14F71427D1E84F3DD0314C0F7B2291E5B200AC8C7C3B925";

    let lexer = Lexer::new(c);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    let contract = parser.parse().unwrap();
    assert_eq!(parser.current_token.kind, TokenKind::Eof);

    // Create const val
    let mut arr: [u8; 32] = Default::default();
    let mut buf =
        BytesMut::from("8C5BE1E5EBEC7D5BD14F71427D1E84F3DD0314C0F7B2291E5B200AC8C7C3B925");
    buf.resize(32, 0);
    arr.copy_from_slice(buf.as_ref());

    // Check Literal
    let fsp_constant = contract.constants[0].clone();
    assert_eq!(fsp_constant, ConstantDefinition { name: "LITERAL", value: ConstVal::Literal(arr) });
}
