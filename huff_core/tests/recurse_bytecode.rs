use huff_codegen::Codegen;
use huff_lexer::*;
use huff_parser::Parser;
use huff_utils::prelude::*;

#[test]
fn recurse_macro_bytecode() {
    let source = r#"
    #define constant TOTAL_SUPPLY_LOCATION = FREE_STORAGE_POINTER()
    #define macro CONSTRUCTOR() = takes(0) returns (0) {}

    #define macro TRANSFER() = takes(0) returns(1) {
      0x04 calldataload
      caller
      0x24 calldataload

      /* THIS FUNCTION DOESN'T ACTUALLY TRANSFER */

      0x01 0x00 mstore
      0x20 0x00 return
    }

    #define macro MINT() = takes(0) returns (0) {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]

        // UPDATE TOTAL SUPPLY
        dup1                             // [value, value, from, to]
        [TOTAL_SUPPLY_LOCATION] sload    // [supply,value,value,from,to]
        add                              // [supply+value,value,from,to]
        [TOTAL_SUPPLY_LOCATION] sstore   // [value,from,to]
    }

    #define macro MAIN() = takes(0) returns (0) {
        0x00 calldataload 0xE0 shr
        dup1 0xa9059cbb eq transfer jumpi
        dup1 0x40c10f19 eq mints jumpi

        transfer:
            TRANSFER()
        mints:
            MINT()
    }
    "#;

    // Lex + Parse
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    let evm_version = &EVMVersion::default();

    // Create main and constructor bytecode
    let main_bytecode = Codegen::generate_main_bytecode(evm_version, &contract, None).unwrap();
    let (constructor_bytecode, has_custom_bootstrap) =
        Codegen::generate_constructor_bytecode(evm_version, &contract, None).unwrap();
    assert!(!has_custom_bootstrap);

    // Full expected bytecode output (generated from huffc) (placed here as a reference)
    let expected_bytecode = "6100398061000d6000396000f35f3560e01c8063a9059cbb1461001b57806340c10f191461002b575b6004353360243560015f5260205ff35b6004355f602435805f54015f55";

    // Construct the expected output
    let mut artifact = Artifact::default();

    // We don't have any constructor args
    let constructor_args = "".to_string();

    // Size config
    let contract_length = main_bytecode.len() / 2;
    let constructor_length = constructor_bytecode.len() / 2;
    let contract_size = format!("{contract_length:04x}");
    let contract_code_offset = format!("{:04x}", 13 + constructor_length);

    // Generate artifact bytecode and runtime code
    let bootstrap_code = format!("61{contract_size}8061{contract_code_offset}6000396000f3");
    let constructor_code = format!("{constructor_bytecode}{bootstrap_code}");
    artifact.bytecode = format!("{constructor_code}{main_bytecode}{constructor_args}");
    artifact.runtime = main_bytecode;

    // Check the bytecode
    assert_eq!(artifact.bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}
