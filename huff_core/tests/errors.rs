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

    let const_start = source.find("UNKNOWN_CONSTANT_DEFINITION").unwrap_or(0);
    let const_end = const_start + "UNKNOWN_CONSTANT_DEFINITION".len();

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let contract = parser.parse().unwrap();

    // Create main and constructor bytecode
    match Codegen::generate_main_bytecode(&contract) {
        Ok(_) => panic!("moose"),
        Err(e) => {
            assert_eq!(
                e,
                CodegenError {
                    kind: CodegenErrorKind::StoragePointersNotDerived,
                    span: AstSpan(vec![
                        Span { start: 5, end: 12, file: None },
                        Span { start: 13, end: 21, file: None },
                        Span { start: 22, end: 43, file: None },
                        Span { start: 44, end: 45, file: None },
                        Span { start: 46, end: 68, file: None }
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
    let const_end = const_start + "UNKNOWN_CONSTANT_DEFINITION".len();

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create main and constructor bytecode
    match Codegen::generate_main_bytecode(&contract) {
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
