use huff_codegen::Codegen;
use huff_lexer::Lexer;
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
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // let contract = Contract {
    //     macros: vec![
    //         MacroDefinition {
    //             name: "CONSTRUCTOR".to_string(),
    //             parameters: vec![],
    //             statements: vec![],
    //             takes: 0,
    //             returns: 0,
    //         },
    //         MacroDefinition {
    //             name: "TRANSFER".to_string(),
    //             parameters: vec![],
    //             statements: vec![
    //                 Statement::Literal(str_to_bytes32("04")),
    //                 Statement::Opcode(Opcode::Calldataload),
    //                 Statement::Opcode(Opcode::Caller),
    //                 Statement::Literal(str_to_bytes32("24")),
    //                 Statement::Opcode(Opcode::Calldataload),
    //                 Statement::Literal(str_to_bytes32("01")),
    //                 Statement::Literal(str_to_bytes32("00")),
    //                 Statement::Opcode(Opcode::Mstore),
    //                 Statement::Literal(str_to_bytes32("20")),
    //                 Statement::Literal(str_to_bytes32("00")),
    //                 Statement::Opcode(Opcode::Return),
    //             ],
    //             takes: 0,
    //             returns: 1,
    //         },
    //         MacroDefinition {
    //             name: "MINT".to_string(),
    //             parameters: vec![],
    //             statements: vec![
    //                 Statement::Literal(str_to_bytes32("04")),
    //                 Statement::Opcode(Opcode::Calldataload),
    //                 Statement::Literal(str_to_bytes32("00")),
    //                 Statement::Literal(str_to_bytes32("24")),
    //                 Statement::Opcode(Opcode::Calldataload),
    //                 Statement::Opcode(Opcode::Dup1),
    //                 Statement::Constant("TOTAL_SUPPLY_LOCATION".to_string()),
    //                 Statement::Opcode(Opcode::Sload),
    //                 Statement::Opcode(Opcode::Add),
    //                 Statement::Constant("TOTAL_SUPPLY_LOCATION".to_string()),
    //                 Statement::Opcode(Opcode::Sstore),
    //             ],
    //             takes: 0,
    //             returns: 0,
    //         },
    //         MacroDefinition {
    //             name: "MAIN".to_string(),
    //             parameters: vec![],
    //             statements: vec![
    //                 Statement::Literal(str_to_bytes32("00")),
    //                 Statement::Opcode(Opcode::Calldataload),
    //                 Statement::Literal(str_to_bytes32("E0")),
    //                 Statement::Opcode(Opcode::Shr),
    //                 Statement::Opcode(Opcode::Dup1),
    //                 Statement::Literal(str_to_bytes32("a9059cbb")),
    //                 Statement::Opcode(Opcode::Eq),
    //                 Statement::LabelCall("transfer".to_string()),
    //                 Statement::Opcode(Opcode::Jumpi),
    //                 Statement::Opcode(Opcode::Dup1),
    //                 Statement::Literal(str_to_bytes32("40c10f19")),
    //                 Statement::Opcode(Opcode::Eq),
    //                 Statement::LabelCall("mints".to_string()),
    //                 Statement::Opcode(Opcode::Jumpi),
    //                 Statement::Label(Label {
    //                     name: "transfer".to_string(),
    //                     inner: vec![Statement::MacroInvocation(MacroInvocation {
    //                         macro_name: "TRANSFER".to_string(),
    //                         args: vec![],
    //                     })],
    //                 }),
    //                 Statement::Label(Label {
    //                     name: "mints".to_string(),
    //                     inner: vec![Statement::MacroInvocation(MacroInvocation {
    //                         macro_name: "MINT".to_string(),
    //                         args: vec![],
    //                     })],
    //                 }),
    //             ],
    //             takes: 0,
    //             returns: 0,
    //         },
    //     ],
    //     invocations: vec![],
    //     imports: vec![],
    //     constants: vec![ConstantDefinition {
    //         name: "TOTAL_SUPPLY_LOCATION".to_string(),
    //         value: ConstVal::FreeStoragePointer(FreeStoragePointer),
    //     }],
    //     functions: vec![],
    //     events: vec![],
    //     tables: vec![],
    // };

    // Sanity Check The AST
    // assert_eq!(contract, ast);

    // Create main and constructor bytecode
    let main_bytecode = Codegen::generate_main_bytecode(&contract).unwrap();
    let constructor_bytecode = Codegen::generate_constructor_bytecode(&contract).unwrap();

    // Full expected bytecode output (generated from huffc) (placed here as a reference)
    let expected_bytecode = "61003f8061000d6000396000f360003560E01c8063a9059cbb1461001c57806340c10f191461002e575b60043533602435600160005260206000f35b60043560006024358060005401600055";

    // Construct the expected output
    let mut artifact = Artifact::default();

    // We don't have any constructor args
    let constructor_args = "".to_string();

    // Size config
    let contract_length = main_bytecode.len() / 2;
    let constructor_length = constructor_bytecode.len() / 2;
    let contract_size = format!("{:04x}", contract_length);
    let contract_code_offset = format!("{:04x}", 13 + constructor_length);

    // Generate artifact bytecode and runtime code
    let bootstrap_code = format!("61{}8061{}6000396000f3", contract_size, contract_code_offset);
    let constructor_code = format!("{}{}", constructor_bytecode, bootstrap_code);
    artifact.bytecode = format!("{}{}{}", constructor_code, main_bytecode, constructor_args);
    artifact.runtime = main_bytecode;

    // Check the bytecode
    assert_eq!(artifact.bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}
