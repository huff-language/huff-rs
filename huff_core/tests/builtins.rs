use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn test_codesize_builtin() {
    let source: &str = r#"
        #define constant OWNER_POINTER = FREE_STORAGE_POINTER()

        #define macro OWNABLE() = takes (0) returns (0) {
            caller [OWNER_POINTER] sstore
        }

        #define macro BUILTIN_TEST() = takes(0) returns(1) {
            __codesize(OWNABLE)
        }

        #define macro CONSTRUCTOR() = takes(0) returns (0) {
            BUILTIN_TEST()
        }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Parse the AST
    let mut contract = parser.parse().unwrap();

    // Derive storage pointers
    contract.derive_storage_pointers();

    // Instantiate Codegen
    let cg = Codegen::new();

    // The codegen instance should have no artifact
    assert!(cg.artifact.is_none());

    // Have the Codegen create the constructor bytecode
    let cbytes = Codegen::generate_constructor_bytecode(&contract).unwrap();
    println!("Constructor Bytecode Result: {:?}", cbytes);
    assert_eq!(cbytes, String::from("6004"));
}

#[test]
fn test_tablesize_builtin() {
    let source: &str = r#"
        #define jumptable__packed PACKED_JUMPTABLE {
            lab_0 lab_1 lab_2 lab_3
        }

        #define jumptable STANDARD_JUMPTABLE {
            lab_0 // 0 00000
            lab_1 // 1 00001
            lab_2 // 2 00010
            lab_3 // 3 00011
        }

        #define macro TEST_INIT_JUMP_TABLE() = takes(0) returns(1) {
            __tablesize(STANDARD_JUMPTABLE) __tablestart(STANDARD_JUMPTABLE) 0x00 codecopy
        }

        #define macro BUILTIN_TEST() = takes(0) returns(1) {
            __tablesize(PACKED_JUMPTABLE)
        }

        #define macro CONSTRUCTOR() = takes(0) returns (0) {
            BUILTIN_TEST()
        }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Parse the AST
    let mut contract = parser.parse().unwrap();

    // Derive storage pointers
    contract.derive_storage_pointers();

    // Instantiate Codegen
    let cg = Codegen::new();

    // The codegen instance should have no artifact
    assert!(cg.artifact.is_none());

    // Have the Codegen create the constructor bytecode
    let cbytes = Codegen::generate_constructor_bytecode(&contract).unwrap();
    println!("Constructor Bytecode Result: {:?}", cbytes);
    assert_eq!(cbytes, String::from("6008"));
}

#[test]
fn test_tablestart_builtin() {
    let source: &str = r#"
        #define jumptable__packed PACKED_JUMPTABLE {
            lab_0 lab_1 lab_2 lab_3
        }

        #define jumptable STANDARD_JUMPTABLE {
            lab_0 // 0 00000
            lab_1 // 1 00001
            lab_2 // 2 00010
            lab_3 // 3 00011
        }

        #define macro BUILTIN_TEST() = takes(0) returns(1) {
            __tablestart(PACKED_JUMPTABLE)
            __tablestart(STANDARD_JUMPTABLE)
        }

        #define macro CONSTRUCTOR() = takes(0) returns (0) {
            BUILTIN_TEST()
        }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Parse the AST
    let mut contract = parser.parse().unwrap();

    // Derive storage pointers
    contract.derive_storage_pointers();

    // Instantiate Codegen
    let cg = Codegen::new();

    // The codegen instance should have no artifact
    assert!(cg.artifact.is_none());

    // Have the Codegen create the constructor bytecode
    let cbytes = Codegen::generate_constructor_bytecode(&contract).unwrap();
    println!("Constructor Bytecode Result: {:?}", cbytes);
    assert_eq!(cbytes, String::from("61xxxx61xxxx"));
}

#[test]
fn test_jump_table_exhaustive_uage() {
    let source: &str = r#"
        #define jumptable STANDARD_JUMPTABLE {
            lab_0 // 0 00000
            lab_1 // 1 00001
            lab_2 // 2 00010
            lab_3 // 3 00011
        }

        // Copies the standard table into memory with codecopy
        #define macro INIT_JUMP_TABLE() = takes(0) returns(1) {
            __tablesize(STANDARD_JUMPTABLE) __tablestart(STANDARD_JUMPTABLE) 0x00 codecopy
        }

        #define macro COMPUTE() = takes (0) returns (1) {
            0x20 dup8 sub mload 0x02ffe0 and
            dup1 0x20 add

            lab_0:
                0x20 0x20 add
            lab_1:
                0x20 0x20 add
            lab_2:
                0x20 0x20 add
            lab_3:
                0x20 0x20 add
        }

        #define macro MAIN() = takes(0) returns (0) {
            0x00 calldataload 0xE0 shr
            dup1 0xa9059cbb eq compute jumpi

            compute:
                COMPUTE()
        }

        #define macro CONSTRUCTOR() = takes(0) returns (0) {
            INIT_JUMP_TABLE()
        }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Parse the AST
    let mut contract = parser.parse().unwrap();

    // Derive storage pointers
    contract.derive_storage_pointers();

    // Instantiate Codegen
    let cg = Codegen::new();

    // The codegen instance should have no artifact
    assert!(cg.artifact.is_none());

    // Have the Codegen create the constructor bytecode
    let cbytes = Codegen::generate_constructor_bytecode(&contract).unwrap();
    let mbytes = Codegen::generate_main_bytecode(&contract).unwrap();
    println!("Constructor Bytecode Result: {:?}", cbytes);
    assert_eq!(cbytes, String::from("61012861xxxx600039"));
    assert_eq!(mbytes, String::from("60003560e01c8063a9059cbb14610011575b60208703516202ffe016806020015b60206020015b60206020015b60206020015b602060200100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000026000000000000000000000000000000000000000000000000000000000000002c0000000000000000000000000000000000000000000000000000000000000032"));
}

#[test]
fn test_jump_table_packed_exhaustive_uage() {
    let source: &str = r#"
        #define jumptable__packed PACKED_JUMPTABLE {
            lab_0 lab_1 lab_2 lab_3
        }

        // Copies the standard table into memory with codecopy
        #define macro INIT_JUMP_TABLE() = takes(0) returns(1) {
            __tablesize(PACKED_JUMPTABLE) __tablestart(PACKED_JUMPTABLE) 0x00 codecopy
        }

        #define macro COMPUTE() = takes (0) returns (1) {
            0x20 dup8 sub mload 0x02ffe0 and
            dup1 0x20 add

            lab_0:
                0x20 0x20 add
            lab_1:
                0x20 0x20 add
            lab_2:
                0x20 0x20 add
            lab_3:
                0x20 0x20 add
        }

        #define macro MAIN() = takes(0) returns (0) {
            0x00 calldataload 0xE0 shr
            dup1 0xa9059cbb eq compute jumpi

            compute:
                COMPUTE()
        }

        #define macro CONSTRUCTOR() = takes(0) returns (0) {
            INIT_JUMP_TABLE()
        }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Parse the AST
    let mut contract = parser.parse().unwrap();

    // Derive storage pointers
    contract.derive_storage_pointers();

    // Instantiate Codegen
    let cg = Codegen::new();

    // The codegen instance should have no artifact
    assert!(cg.artifact.is_none());

    // Have the Codegen create the constructor bytecode
    let cbytes = Codegen::generate_constructor_bytecode(&contract).unwrap();
    let mbytes = Codegen::generate_main_bytecode(&contract).unwrap();
    println!("Constructor Bytecode Result: {:?}", cbytes);
    assert_eq!(cbytes, String::from("600861xxxx600039"));
    assert_eq!(mbytes, String::from("60003560e01c8063a9059cbb14610011575b60208703516202ffe016806020015b60206020015b60206020015b60206020015b602060200100200026002c0032"));
}
