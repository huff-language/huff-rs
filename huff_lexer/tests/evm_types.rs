use huff_lexer::*;
use huff_utils::{
    prelude::*,
    types::*,
};

#[test]
fn primitive_type_parsing() {
    let evm_types = [
        ("address", PrimitiveEVMType::Address),
        ("string", PrimitiveEVMType::String),
        ("uint192", PrimitiveEVMType::Uint(192)),
        ("bytes32", PrimitiveEVMType::Bytes(32)),
        ("bool", PrimitiveEVMType::Bool),
        ("int8", PrimitiveEVMType::Int(8)),
        ("bytes", PrimitiveEVMType::DynBytes),
    ];

    for (evm_type, evm_type_enum) in evm_types {
        let source = format!("#define function test({}) view returns (uint256)", evm_type);
        let mut lexer = Lexer::new(source.as_str());
        let tokens = lexer
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
        .collect::<Vec<Token>>();

        assert_eq!(tokens.get(5).unwrap().kind, TokenKind::EVMType(EVMType::Primitive(evm_type_enum)));
    }
}
