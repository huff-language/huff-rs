use std::{collections::HashMap, sync::Arc};

use huff_core::Compiler;
use huff_utils::prelude::EVMVersion;

#[test]
fn test_in_memory_compiler() {
    let source_main = r#"
    #include "../lib/mint.huff"

    #define macro MAIN() = takes(0) returns (0) {
        0x00 calldataload 0xE0 shr
        dup1 0x40c10f19 eq mints jumpi

        mints:
            MINT()
    }
    "#;

    let source_mint = r#"
    #define macro MINT() = takes(0) returns (0) {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]
    }
    "#;

    let main_file_name = String::from("contracts/main.huff");

    let mut file_sources = HashMap::new();
    file_sources.insert(main_file_name.clone(), String::from(source_main));
    file_sources.insert(String::from("lib/mint.huff"), String::from(source_mint));

    // Instantiate a new compiler
    let evm_version = EVMVersion::default();
    let compiler = Compiler::new_in_memory(
        &evm_version,
        Arc::new(vec![main_file_name.clone()]),
        file_sources,
        None,
        None,
        None,
        None,
        false,
    );

    let result = compiler.execute();
    assert!(result.is_ok());

    let artifacts = result.unwrap();
    let artifact = artifacts.iter().find(|a| main_file_name.eq(&a.file.path)).unwrap();

    assert_eq!(
        artifact.bytecode,
        "60188060093d393df35f3560e01c806340c10f1914610010575b6004355f602435".to_string()
    );
}
