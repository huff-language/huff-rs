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

        #define table CODE_TABLE {
            0xDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEF
        }

        #define macro TEST_INIT_JUMP_TABLE() = takes(0) returns(1) {
            __tablesize(STANDARD_JUMPTABLE) __tablestart(STANDARD_JUMPTABLE) 0x00 codecopy
        }

        #define macro BUILTIN_TEST() = takes(0) returns(1) {
            __tablesize(PACKED_JUMPTABLE)
        }

        #define macro BUILTIN_TEST_2() = takes(0) returns(1) {
            __tablesize(CODE_TABLE)
        }

        #define macro MAIN() = takes(0) returns (0) {
            BUILTIN_TEST()
            TEST_INIT_JUMP_TABLE()
            BUILTIN_TEST_2()

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
    assert_eq!(mbytes, String::from("6008608061002c60003960205b60006000f35b60006000f35b60006000f35b60006000f3000c00120018001e000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000120000000000000000000000000000000000000000000000000000000000000018000000000000000000000000000000000000000000000000000000000000001eDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEFDEADBEEF"));
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

        #define macro BUILTIN_TEST() = takes(0) returns(2) {
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
    assert_eq!(mbytes, String::from("608061004060003960003560e01c8063a9059cbb14610019575b60208703516202ffe016806020015b60206020015b60206020015b60206020015b60206020010000000000000000000000000000000000000000000000000000000000000028000000000000000000000000000000000000000000000000000000000000002e0000000000000000000000000000000000000000000000000000000000000034000000000000000000000000000000000000000000000000000000000000003a"));
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
    assert_eq!(mbytes, String::from("6008610048600039608061005060003960003560e01c8063a9059cbb14610021575b60208703516202ffe016806020015b60206020015b60206020015b60206020015b602060200100300036003c004200000000000000000000000000000000000000000000000000000000000000300000000000000000000000000000000000000000000000000000000000000036000000000000000000000000000000000000000000000000000000000000003c0000000000000000000000000000000000000000000000000000000000000042"));
}

#[test]
fn test_func_sig_builtin() {
    let source: &str = r#"
        #define function transfer(address,uint256) nonpayable returns ()

        #define macro TRANSFER() = takes (0) returns (0) {
            // ...
        }

        #define macro MAIN() = takes(0) returns (0) {
            // Identify which function is being called.
            0x00 calldataload 0xE0 shr
            dup1 __FUNC_SIG("transfer(address,uint256)") eq transfer jumpi
            dup1 __FUNC_SIG('transfer(address,uint256)') eq transfer jumpi
            dup1 __FUNC_SIG(transfer) eq transfer jumpi

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
    // `transfer(address,uint256) signature = 0xa9059cbb
    assert_eq!(&cbytes[16..24], "a9059cbb");
    assert_eq!(&cbytes[38..46], "a9059cbb");
    assert_eq!(&cbytes[60..68], "a9059cbb");
    assert_eq!(
        cbytes,
        String::from(
            "60003560e01c8063a9059cbb14610027578063a9059cbb14610027578063a9059cbb14610027575b"
        )
    );
}

#[test]
fn test_event_hash_builtin() {
    let source: &str = r#"
        #define event transfer(address,address,uint256)

        #define macro MAIN() = takes(0) returns (0) {
            __EVENT_HASH("transfer(address,address,uint256)")
            __EVENT_HASH('transfer(address,address,uint256)')
            __EVENT_HASH(transfer)
            0x00 sstore
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
    // `transfer(address,address,uint256) signature =
    // 0xbeabacc8ffedac16e9a60acdb2ca743d80c2ebb44977a93fa8e483c74d2b35a8
    assert_eq!(&cbytes[2..66], "beabacc8ffedac16e9a60acdb2ca743d80c2ebb44977a93fa8e483c74d2b35a8");
    assert_eq!(
        &cbytes[68..132],
        "beabacc8ffedac16e9a60acdb2ca743d80c2ebb44977a93fa8e483c74d2b35a8"
    );
    assert_eq!(
        &cbytes[134..198],
        "beabacc8ffedac16e9a60acdb2ca743d80c2ebb44977a93fa8e483c74d2b35a8"
    );
    assert_eq!(
        cbytes,
        String::from("7fbeabacc8ffedac16e9a60acdb2ca743d80c2ebb44977a93fa8e483c74d2b35a87fbeabacc8ffedac16e9a60acdb2ca743d80c2ebb44977a93fa8e483c74d2b35a87fbeabacc8ffedac16e9a60acdb2ca743d80c2ebb44977a93fa8e483c74d2b35a8600055")
    );
}

#[test]
fn test_error_selector_builtin() {
    let source: &str = r#"
        // Define our custom error
        #define error PanicError(uint256 panicCode)
        #define error Error(string)

        #define macro PANIC() = takes (1) returns (0) {
            // Input stack:          [panic_code]
            __ERROR(PanicError)   // [panic_error_selector, panic_code]
            0x00 mstore           // [panic_code]
            0x04 mstore           // []
            0x24 0x00 revert
        }

        #define macro REQUIRE() = takes (3) returns (0) {
            // Input stack:          [condition, message_length, message]
            continue jumpi        // [message]

            __ERROR(Error)        // [error_selector, message_length, message]
            0x00 mstore           // [message_length, message]
            0x20 0x04 mstore      // [message_length, message]
            0x24 mstore           // [message]
            0x44 mstore           // []

            0x64 0x00 revert

            continue:
                pop               // []
        }

        #define macro MAIN() = takes (0) returns (0) {
            // dummy macro invocations so they're included in the runtime bytecode
            PANIC()
            REQUIRE()
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

    // Have Codegen create the runtime bytecode
    let r_bytes = Codegen::generate_main_bytecode(&contract).unwrap();
    assert_eq!(&r_bytes[2..66], "be20788c00000000000000000000000000000000000000000000000000000000");
    assert_eq!(
        &r_bytes[98..162],
        "08c379a000000000000000000000000000000000000000000000000000000000"
    );
    assert_eq!(
        r_bytes,
        String::from(
            "7fbe20788c0000000000000000000000000000000000000000000000000000000060005260045260246000fd610064577f08c379a000000000000000000000000000000000000000000000000000000000600052602060045260245260445260646000fd5b50"
        )
    );
}

#[test]
fn test_rightpad_builtin() {
    let source: &str = r#"
        #define macro MAIN() = takes (0) returns (0) {
            __RIGHTPAD(0xa57b)
            __RIGHTPAD(0x48656c6c6f2c20576f726c6421)
            __RIGHTPAD(0x6d6f6f7365)
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

    // Have Codegen create the runtime bytecode
    let r_bytes = Codegen::generate_main_bytecode(&contract).unwrap();
    assert_eq!(&r_bytes[2..66], "a57b000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(
        &r_bytes[68..132],
        "48656c6c6f2c20576f726c642100000000000000000000000000000000000000"
    );
    assert_eq!(
        &r_bytes[134..198],
        "6d6f6f7365000000000000000000000000000000000000000000000000000000"
    );
    assert_eq!(r_bytes.len(), (32 * 3 + 3) * 2);
    assert_eq!(
        r_bytes,
        String::from(
            "7fa57b0000000000000000000000000000000000000000000000000000000000007f48656c6c6f2c20576f726c6421000000000000000000000000000000000000007f6d6f6f7365000000000000000000000000000000000000000000000000000000"
        )
    );
}
