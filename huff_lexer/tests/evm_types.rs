use huff_lexer::*;
use huff_utils::prelude::*;

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
        let source = &format!("#define function test({evm_type}) view returns (uint256)");
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(flattened_source.source.clone());
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
        ("address[3]", TokenKind::ArrayType(PrimitiveEVMType::Address, vec![3])),
        ("string[1]", TokenKind::ArrayType(PrimitiveEVMType::String, vec![1])),
        ("uint192[4]", TokenKind::ArrayType(PrimitiveEVMType::Uint(192), vec![4])),
        ("bytes32[11]", TokenKind::ArrayType(PrimitiveEVMType::Bytes(32), vec![11])),
        ("bool[2]", TokenKind::ArrayType(PrimitiveEVMType::Bool, vec![2])),
        ("int8[3]", TokenKind::ArrayType(PrimitiveEVMType::Int(8), vec![3])),
        ("bytes[6]", TokenKind::ArrayType(PrimitiveEVMType::DynBytes, vec![6])),
    ];

    for (evm_type, evm_type_enum) in evm_types {
        let source = &format!("#define function test({evm_type}) view returns (uint256)");
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(flattened_source.source.clone());
        let tokens = lexer
            .into_iter()
            .map(|x| x.unwrap())
            .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
            .collect::<Vec<Token>>();

        assert_eq!(tokens.get(4).unwrap().kind, evm_type_enum);
    }
}

#[test]
fn unbounded_array_parsing() {
    let evm_types = [
        ("address[]", TokenKind::ArrayType(PrimitiveEVMType::Address, vec![0])),
        ("string[]", TokenKind::ArrayType(PrimitiveEVMType::String, vec![0])),
        ("uint192[]", TokenKind::ArrayType(PrimitiveEVMType::Uint(192), vec![0])),
        ("bytes32[]", TokenKind::ArrayType(PrimitiveEVMType::Bytes(32), vec![0])),
        ("bool[]", TokenKind::ArrayType(PrimitiveEVMType::Bool, vec![0])),
        ("int8[]", TokenKind::ArrayType(PrimitiveEVMType::Int(8), vec![0])),
        ("bytes[]", TokenKind::ArrayType(PrimitiveEVMType::DynBytes, vec![0])),
    ];

    for (evm_type, evm_type_enum) in evm_types {
        let source = &format!("#define function test({evm_type}) view returns (uint256)");
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(flattened_source.source.clone());
        let tokens = lexer
            .into_iter()
            .map(|x| x.unwrap())
            .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
            .collect::<Vec<Token>>();
        assert_eq!(tokens.get(4).unwrap().kind, evm_type_enum);
    }
}

#[test]
fn multidim_array_parsing() {
    let evm_types = [
        ("address[3][2]", TokenKind::ArrayType(PrimitiveEVMType::Address, vec![3, 2])),
        ("string[1][]", TokenKind::ArrayType(PrimitiveEVMType::String, vec![1, 0])),
        ("uint192[][][]", TokenKind::ArrayType(PrimitiveEVMType::Uint(192), vec![0, 0, 0])),
        ("bytes32[][11]", TokenKind::ArrayType(PrimitiveEVMType::Bytes(32), vec![0, 11])),
        ("bool[2][4]", TokenKind::ArrayType(PrimitiveEVMType::Bool, vec![2, 4])),
        ("int8[3][4]", TokenKind::ArrayType(PrimitiveEVMType::Int(8), vec![3, 4])),
        ("bytes[6][4]", TokenKind::ArrayType(PrimitiveEVMType::DynBytes, vec![6, 4])),
    ];

    for (evm_type, evm_type_enum) in evm_types {
        let source = &format!("#define function test({evm_type}) view returns (uint256)");
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(flattened_source.source.clone());
        let tokens = lexer
            .into_iter()
            .map(|x| x.unwrap())
            .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
            .collect::<Vec<Token>>();

        assert_eq!(tokens.get(4).unwrap().kind, evm_type_enum);
    }
}
