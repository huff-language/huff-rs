use bytes::BytesMut;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn derives_storage_pointers() {
    let c =
        "#define constant FSP_LOCATION = FREE_STORAGE_POINTER()\n#define constant FSP_LOCATION_2 = FREE_STORAGE_POINTER()\n#define constant NUM = 0xa57B";

    let lexer = Lexer::new(c);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    let mut contract = parser.parse().unwrap();
    assert_eq!(parser.current_token.kind, TokenKind::Eof);

    // Ensure that the constant definitions were parsed correctly
    let fsp_constant = contract.constants[0].clone();
    assert_eq!(
        fsp_constant,
        ConstantDefinition {
            name: "FSP_LOCATION",
            value: ConstVal::FreeStoragePointer(FreeStoragePointer {})
        }
    );

    let fsp_constant = contract.constants[1].clone();
    assert_eq!(
        fsp_constant,
        ConstantDefinition {
            name: "FSP_LOCATION_2",
            value: ConstVal::FreeStoragePointer(FreeStoragePointer {})
        }
    );

    let num_constant = contract.constants[2].clone();
    assert_eq!(
        num_constant,
        ConstantDefinition { name: "NUM", value: ConstVal::Literal(str_to_bytes32("a57B")) }
    );
    // Ensure that storage pointers were derived correctly
    let storage_pointers = contract.derive_storage_pointers().unwrap();
    assert_eq!(storage_pointers[0], str_to_bytes32("a57B")); // 0xa57B
    assert_eq!(storage_pointers[1], str_to_bytes32("0")); // FSP
    assert_eq!(storage_pointers[2], str_to_bytes32("1")); // FSP #2

    // Ensure that the storage pointers were set for the FSP constants in the AST
    assert_eq!(contract.constants[0].value, ConstVal::Literal(str_to_bytes32("0")));
    assert_eq!(contract.constants[1].value, ConstVal::Literal(str_to_bytes32("1")));
    assert_eq!(contract.constants[2].value, ConstVal::Literal(str_to_bytes32("a57B")));
}
