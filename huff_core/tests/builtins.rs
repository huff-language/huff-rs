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

        #define macro MAIN() = takes(0) returns (0) {
            BUILTIN_TEST()

            lab_0:
                0x00
                0x00
                return
            lab_1:
                0x00
                0x00
                return
            lab_2:
                0x00
                0x00
                return
            lab_3:
                0x00
                0x00
                return
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
    let mbytes = Codegen::generate_main_bytecode(&contract).unwrap();
    assert_eq!(mbytes, String::from("60085b60006000f35b60006000f35b60006000f35b60006000f300020008000e001400000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000014"));
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

            lab_0:
                0x00
                0x00
                return
            lab_1:
                0x00
                0x00
                return
            lab_2:
                0x00
                0x00
                return
            lab_3:
                0x00
                0x00
                return
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
    assert_eq!(cbytes, String::from("61001e6100265b60006000f35b60006000f35b60006000f35b60006000f30006000c001200180000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000120000000000000000000000000000000000000000000000000000000000000018"));
}

#[test]
fn test_jump_table_exhaustive_usage() {
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
            INIT_JUMP_TABLE()

            0x00 calldataload 0xE0 shr
            dup1 0xa9059cbb eq compute jumpi

            compute:
                COMPUTE()
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
    let mbytes = Codegen::generate_main_bytecode(&contract).unwrap();
    assert_eq!(mbytes, String::from("61012861004160003960003560e01c8063a9059cbb1461001a575b60208703516202ffe016806020015b60206020015b60206020015b60206020015b60206020010000000000000000000000000000000000000000000000000000000000000029000000000000000000000000000000000000000000000000000000000000002f0000000000000000000000000000000000000000000000000000000000000035000000000000000000000000000000000000000000000000000000000000003b"));
}

#[test]
fn test_jump_table_packed_exhaustive_usage() {
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
            INIT_JUMP_TABLE()

            0x00 calldataload 0xE0 shr
            dup1 0xa9059cbb eq compute jumpi

            compute:
                COMPUTE()
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

    // Have the Codegen create the main macro bytecode
    let mbytes = Codegen::generate_main_bytecode(&contract).unwrap();
    assert_eq!(mbytes, String::from("600861004060003960003560e01c8063a9059cbb14610019575b60208703516202ffe016806020015b60206020015b60206020015b60206020015b60206020010028002e0034003a"));
}

#[test]
fn test_label_clashing() {
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

        #define macro INIT_JUMP_TABLES() = takes(0) returns(1) {
            __tablesize(PACKED_JUMPTABLE) __tablestart(PACKED_JUMPTABLE) 0x00 codecopy
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
            INIT_JUMP_TABLES()

            0x00 calldataload 0xE0 shr
            dup1 0xa9059cbb eq compute jumpi

            compute:
                COMPUTE()
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

    // Have the Codegen create the main macro bytecode
    let mbytes = Codegen::generate_main_bytecode(&contract).unwrap();
    assert_eq!(mbytes, String::from("600861004960003961012861005160003960003560e01c8063a9059cbb14610022575b60208703516202ffe016806020015b60206020015b60206020015b60206020015b602060200100310037003d004300000000000000000000000000000000000000000000000000000000000000310000000000000000000000000000000000000000000000000000000000000037000000000000000000000000000000000000000000000000000000000000003d0000000000000000000000000000000000000000000000000000000000000043"));
}

#[test]
fn test_sig_builtin() {
    let source: &str = r#"
        #define function transfer(address,uint256) nonpayable returns ()

        #define macro TRANSFER() = takes (0) returns (0) {
            // ...
        }

        #define macro MAIN() = takes(0) returns (0) {
            // Identify which function is being called.
            0x00 calldataload 0xE0 shr
            dup1 __SIG(transfer) eq transfer jumpi

            transfer:
                TRANSFER()
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
    let cbytes = Codegen::generate_main_bytecode(&contract).unwrap();
    println!("Main Bytecode Result: {:?}", cbytes);
    assert_eq!(&cbytes[16..24], "a9059cbb"); // `transfer(address,uint256) signature = 0xa9059cbb
    assert_eq!(cbytes, String::from("60003560e01c8063a9059cbb14610011575b"));
}
