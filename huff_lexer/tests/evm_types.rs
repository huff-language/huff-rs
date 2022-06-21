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
        let tokens = lexer.into_iter().collect::<Vec<Token>>();

        assert_eq!(tokens.get(4).unwrap().kind, TokenKind::PrimitiveType(evm_type));
    }
}

#[test]
fn bounded_array_parsing() {
    let evm_types = [
        ("address[3]", TokenKind::ArrayType("address[3]")),
        ("string[1]", TokenKind::ArrayType("string[1]")),
        ("uint192[4]", TokenKind::ArrayType("uint192[4]")),
        ("bytes32[11]", TokenKind::ArrayType("bytes32[11]")),
        ("bool[2]", TokenKind::ArrayType("bool[2]")),
        ("int8[3]", TokenKind::ArrayType("int8[3]")),
        ("bytes[6]", TokenKind::ArrayType("bytes[6]")),
    ];

    for (evm_type, evm_type_enum) in evm_types {
        let source = format!("#define function test({}) view returns (uint256)", evm_type);
        let lexer = Lexer::new(source.as_str());
        let tokens = lexer.into_iter().collect::<Vec<Token>>();

        assert_eq!(tokens.get(4).unwrap().kind, evm_type_enum);
    }
}

#[test]
fn unbounded_array_parsing() {
    let evm_types = [
        ("address[]", TokenKind::ArrayType("address[]")),
        ("string[]", TokenKind::ArrayType("string[]")),
        ("uint192[]", TokenKind::ArrayType("uint192[]")),
        ("bytes32[]", TokenKind::ArrayType("bytes32[]")),
        ("bool[]", TokenKind::ArrayType("bool[]")),
        ("int8[]", TokenKind::ArrayType("int8[]")),
        ("bytes[]", TokenKind::ArrayType("bytes[]")),
    ];

    for (evm_type, evm_type_enum) in evm_types {
        let source = format!("#define function test({}) view returns (uint256)", evm_type);
        let lexer = Lexer::new(source.as_str());
        let tokens = lexer.into_iter().collect::<Vec<Token>>();
        assert_eq!(tokens.get(4).unwrap().kind, evm_type_enum);
    }
}

#[test]
fn multidim_array_parsing() {
    let evm_types = [
        ("address[3][2]", TokenKind::ArrayType("address[3][2]")),
        ("string[1][]", TokenKind::ArrayType("string[1][]")),
        ("uint192[][][]", TokenKind::ArrayType("uint192[][][]")),
        ("bytes32[][11]", TokenKind::ArrayType("bytes32[][11]")),
        ("bool[2][4]", TokenKind::ArrayType("bool[2][4]")),
        ("int8[3][4]", TokenKind::ArrayType("int8[3][4]")),
        ("bytes[6][4]", TokenKind::ArrayType("bytes[6][4]")),
    ];

    for (evm_type, evm_type_enum) in evm_types {
        let source = format!("#define function test({}) view returns (uint256)", evm_type);
        let lexer = Lexer::new(source.as_str());
        let tokens = lexer.into_iter().collect::<Vec<Token>>();

        assert_eq!(tokens.get(4).unwrap().kind, evm_type_enum);
    }
}
