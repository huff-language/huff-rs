use huff_lexer::*;
use huff_utils::{prelude::*, types::*};

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
        let lexer = Lexer::new(source.as_str());
        let tokens = lexer
            .into_iter()
            .map(|x| x.unwrap())
            .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
            .collect::<Vec<Token>>();

        assert_eq!(tokens.get(4).unwrap().kind, TokenKind::PrimitiveType(evm_type_enum));
    }
}

#[test]
fn bounded_array_parsing() {
    let evm_types = [
        ("address[3]", TokenKind::ArrayType(PrimitiveEVMType::Address, 3)),
        ("string[1]", TokenKind::ArrayType(PrimitiveEVMType::String, 1)),
        ("uint192[4]", TokenKind::ArrayType(PrimitiveEVMType::Uint(192), 4)),
        ("bytes32[11]", TokenKind::ArrayType(PrimitiveEVMType::Bytes(32), 11)),
        ("bool[2]", TokenKind::ArrayType(PrimitiveEVMType::Bool, 2)),
        ("int8[3]", TokenKind::ArrayType(PrimitiveEVMType::Int(8), 3)),
        ("bytes[6]", TokenKind::ArrayType(PrimitiveEVMType::DynBytes, 6)),
    ];

    for (evm_type, evm_type_enum) in evm_types {
        let source = format!("#define function test({}) view returns (uint256)", evm_type);
        let lexer = Lexer::new(source.as_str());
        let tokens = lexer
            .into_iter()
            .map(|x| x.unwrap())
            .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
            .collect::<Vec<Token>>();

        assert_eq!(tokens.get(4).unwrap().kind, evm_type_enum);
    }
}
