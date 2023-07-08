use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn test_storage_pointers_not_derived() {
    let source = r#"
    #define constant TOTAL_SUPPLY_LOCATION = FREE_STORAGE_POINTER()
    #define macro CONSTRUCTOR() = takes(0) returns (0) {}

    #define macro MINT() = takes(0) returns (0) {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]

        dup1                             // [value, value, from, to]
        [TOTAL_SUPPLY_LOCATION] sload    // [supply,value,value,from,to]
        add                              // [supply+value,value,from,to]
        [TOTAL_SUPPLY_LOCATION] sstore   // [value,from,to]
    }

    #define macro MAIN() = takes(0) returns (0) {
        0x00 calldataload 0xE0 shr
        dup1 0x40c10f19 eq mints jumpi

        mints:
            MINT()
    }
  "#;

    // let const_start = source.find("UNKNOWN_CONSTANT_DEFINITION").unwrap_or(0);
    // let const_end = const_start + "UNKNOWN_CONSTANT_DEFINITION".len();

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let contract = parser.parse().unwrap();

    // Create main and constructor bytecode
    match Codegen::generate_main_bytecode(&EVMVersion::default(), &contract, None) {
        Ok(_) => panic!("moose"),
        Err(e) => {
            assert_eq!(
                e,
                CodegenError {
                    kind: CodegenErrorKind::StoragePointersNotDerived,
                    span: AstSpan(vec![
                        Span { start: 5, end: 11, file: None },
                        Span { start: 13, end: 20, file: None },
                        Span { start: 22, end: 42, file: None },
                        Span { start: 44, end: 44, file: None },
                        Span { start: 46, end: 67, file: None }
                    ]),
                    token: None
                }
            )
        }
    }
}

#[test]
fn test_invalid_constant_definition() {
    let source = r#"
    #define constant TOTAL_SUPPLY_LOCATION = FREE_STORAGE_POINTER()
    #define macro CONSTRUCTOR() = takes(0) returns (0) {}

    #define macro MINT() = takes(0) returns (0) {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]

        [UNKNOWN_CONSTANT_DEFINITION] sload

        dup1                             // [value, value, from, to]
        [TOTAL_SUPPLY_LOCATION] sload    // [supply,value,value,from,to]
        add                              // [supply+value,value,from,to]
        [TOTAL_SUPPLY_LOCATION] sstore   // [value,from,to]
    }

    #define macro MAIN() = takes(0) returns (0) {
        0x00 calldataload 0xE0 shr
        dup1 0x40c10f19 eq mints jumpi

        mints:
            MINT()
    }
  "#;

    let const_start = source.find("UNKNOWN_CONSTANT_DEFINITION").unwrap_or(0);
    let const_end = const_start + "UNKNOWN_CONSTANT_DEFINITION".len() - 1;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create main and constructor bytecode
    match Codegen::generate_main_bytecode(&EVMVersion::default(), &contract, None) {
        Ok(_) => panic!("moose"),
        Err(e) => {
            assert_eq!(
                e,
                CodegenError {
                    kind: CodegenErrorKind::MissingConstantDefinition(
                        "UNKNOWN_CONSTANT_DEFINITION".to_string()
                    ),
                    span: AstSpan(vec![Span { start: const_start, end: const_end, file: None }]),
                    token: None
                }
            )
        }
    }
}

#[test]
fn test_missing_constructor() {
    let source = r#"
    #define macro MINT() = takes(0) returns (0) {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]
    }

    #define macro MAIN() = takes(0) returns (0) {
        0x00 calldataload 0xE0 shr
        dup1 0x40c10f19 eq mints jumpi

        mints:
            MINT()
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create constructor bytecode
    match Codegen::generate_constructor_bytecode(&EVMVersion::default(), &contract, None) {
        Ok(_) => panic!("moose"),
        Err(e) => {
            assert_eq!(
                e,
                CodegenError {
                    kind: CodegenErrorKind::MissingMacroDefinition("CONSTRUCTOR".to_string()),
                    span: AstSpan(vec![Span { start: 0, end: 0, file: None }]),
                    token: None
                }
            )
        }
    }
}

#[test]
fn test_missing_main() {
    let source = r#"
    #define macro MINT() = takes(0) returns (0) {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Createconstructor bytecode
    match Codegen::generate_main_bytecode(&EVMVersion::default(), &contract, None) {
        Ok(_) => panic!("moose"),
        Err(e) => {
            assert_eq!(
                e,
                CodegenError {
                    kind: CodegenErrorKind::MissingMacroDefinition("MAIN".to_string()),
                    span: AstSpan(vec![Span { start: 0, end: 0, file: None }]),
                    token: None
                }
            )
        }
    }
}

#[test]
fn test_missing_when_alternative_main_provided() {
    let source = r#"
    #define macro MINT() = takes(0) returns (0) {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    let alternative_main = Some(String::from("NAH"));

    // Createconstructor bytecode
    match Codegen::generate_main_bytecode(&EVMVersion::default(), &contract, alternative_main) {
        Ok(_) => panic!("moose"),
        Err(e) => {
            assert_eq!(
                e,
                CodegenError {
                    kind: CodegenErrorKind::MissingMacroDefinition("NAH".to_string()),
                    span: AstSpan(vec![Span { start: 0, end: 0, file: None }]),
                    token: None
                }
            )
        }
    }
}

#[test]
fn test_unknown_macro_definition() {
    let source = r#"
    #define macro MINT() = takes(0) returns (0) {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]
    }

    #define macro MAIN() = takes(0) returns (0) {
        0x00 calldataload 0xE0 shr
        dup1 0x40c10f19 eq mints jumpi

        mints:
            UNKNOWN()
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create main and constructor bytecode
    match Codegen::generate_main_bytecode(&EVMVersion::default(), &contract, None) {
        Ok(_) => panic!("moose"),
        Err(e) => {
            assert_eq!(
                e,
                CodegenError {
                    kind: CodegenErrorKind::InvalidMacroInvocation("UNKNOWN".to_string()),
                    span: AstSpan(vec![
                        Span { start: 344, end: 350, file: None },
                        Span { start: 351, end: 351, file: None },
                        Span { start: 352, end: 352, file: None }
                    ]),
                    token: None
                }
            )
        }
    }
}

#[test]
fn test_unmatched_jump_label() {
    let source = r#"
    #define macro MINT(error) = takes(0) returns (0) {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]

        <error> jumpi
    }

    #define macro MAIN() = takes(0) returns (0) {
        0x00 calldataload 0xE0 shr
        dup1 0x40c10f19 eq mints jumpi

        mints:
            MINT(err)
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create main and constructor bytecode
    match Codegen::generate_main_bytecode(&EVMVersion::default(), &contract, None) {
        Ok(_) => panic!("moose"),
        Err(e) => {
            assert_eq!(
                e,
                CodegenError {
                    kind: CodegenErrorKind::UnmatchedJumpLabel,
                    span: AstSpan(vec![
                        Span { start: 372, end: 375, file: None },
                        Span { start: 376, end: 376, file: None },
                        Span { start: 377, end: 379, file: None },
                        Span { start: 380, end: 380, file: None }
                    ]),
                    token: None
                }
            )
        }
    }
}
