use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn derives_storage_pointers() {
    let c =
        "#define constant FSP_LOCATION = FREE_STORAGE_POINTER()\n#define constant FSP_LOCATION_2 = FREE_STORAGE_POINTER()\n#define constant NUM = 0xa57B";

    let lexer = Lexer::new(c);
    let tokens = lexer.into_iter().collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    let mut contract = parser.parse().unwrap();

    // Ensure that the constant definitions were parsed correctly
    let fsp_constant = contract.constants[0].clone();
    assert_eq!(
        fsp_constant,
        ConstantDefinition {
            name: "FSP_LOCATION".to_string(),
            value: ConstVal::FreeStoragePointer(FreeStoragePointer {})
        }
    );

    let fsp_constant = contract.constants[1].clone();
    assert_eq!(
        fsp_constant,
        ConstantDefinition {
            name: "FSP_LOCATION_2".to_string(),
            value: ConstVal::FreeStoragePointer(FreeStoragePointer {})
        }
    );

    let num_constant = contract.constants[2].clone();
    assert_eq!(
        num_constant,
        ConstantDefinition {
            name: "NUM".to_string(),
            value: ConstVal::Literal(str_to_bytes32("a57B"))
        }
    );
    // Derive the AST's free storage pointers
    contract.derive_storage_pointers();
    // Ensure that the storage pointers were set for the FSP constants in the AST
    assert_eq!(contract.constants[0].value, ConstVal::Literal(str_to_bytes32("0")));
    assert_eq!(contract.constants[1].value, ConstVal::Literal(str_to_bytes32("1")));
    assert_eq!(contract.constants[2].value, ConstVal::Literal(str_to_bytes32("a57B")));
}
