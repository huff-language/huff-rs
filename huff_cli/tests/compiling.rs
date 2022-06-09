//! Code Generation
//!
//! Tests the full compilation process from lexing to parsing to code generation.

// use ethers::abi::Token;
use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

const SOURCE: &str = r#"
    /* Storage */
    #define constant OWNER_POINTER = FREE_STORAGE_POINTER()

    /* OWNABLE CONSTRUCTOR */
    #define macro OWNABLE_CONSTRUCTOR() = takes (0) returns (0) {
        caller [OWNER_POINTER] sstore
    }

    /* Methods */
    #define macro OWNABLE_SET_OWNER() = takes (1) returns (0) {
        [OWNER_POINTER] sstore
    }

    #define macro OWNABLE_GET_OWNER() = takes (0) returns (1) {
        [OWNER_POINTER] sload
    }

    // #define macro ONLY_OWNER() = takes(0) returns(0) {
    //     [OWNER_POINTER] sload caller eq is_owner jumpi
    //         0x00 0x00 revert
    //     is_owner:
    // }

    /* Constructor */
    #define macro CONSTRUCTOR() = takes(0) returns (0) {
        // Set msg.sender as the owner of the contract.
        OWNABLE_CONSTRUCTOR()
    }
"#;

#[test]
fn compiles_constructor_bytecode() {
    // Lex and Parse the source code
    let lexer = Lexer::new(SOURCE);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);

    // Grab the first macro
    let contract = parser.parse().unwrap();

    // Instantiate Codegen
    let mut cg = Codegen::new();

    // The codegen instance should have no artifact
    assert!(cg.artifact.is_none());

    // Have the Codegen create the constructor bytecode
    let cbytes = cg.construct(Some(contract)).unwrap();
    println!("Constructor Bytecode Result: {:?}", cbytes);
    assert_eq!(cbytes, String::from("33600055"));
}

#[test]
fn compiles_runtime_bytecode() {
    // Lex and Parse the source code
    let lexer = Lexer::new(SOURCE);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);

    // Grab the first macro
    let contract = parser.parse().unwrap();

    // Instantiate Codegen
    let mut cg = Codegen::new();

    // The codegen instance should have no artifact
    assert!(cg.artifact.is_none());

    // Have the Codegen create the constructor bytecode
    // let cbytes = cg.construct(Some(contract)).unwrap();
    // println!("Constructor Bytecode Result: {:?}", cbytes);
    // assert_eq!(cbytes, String::from("33600055"));

    // // Churn Contract using the bytecode
    // let inputs: Vec<Token> = vec![];
    // // ERC20 Bytecode
    // let main_bytecode =
    // "60003560E01c8063a9059cbb1461004857806340c10f19146100de57806370a082311461014e57806318160ddd1461016b578063095ea7b314610177578063dd62ed3e1461018e575b600435336024358160016000526000602001526040600020548082116100d8578190038260016000526000602001526040600020558281906001600052600060200152604060002054018360016000526000602001526040600020556000527fDDF252AD1BE2C89B69C2B068FC378DAA952BA7F163C4A11628F55A4DF523B3EF60206000a3600160005260206000f35b60006000fd5b60005433146100ed5760006000fd5b600435600060243582819060016000526000602001526040600020540183600160005260006020015260406000205580600254016002556000527fDDF252AD1BE2C89B69C2B068FC378DAA952BA7F163C4A11628F55A4DF523B3EF60206000a35b600435600160005260006020015260406000205460005260206000f35b60025460005260206000f35b602435600435336000526000602001526040600020555b60243560043560005260006020015260406000205460005260206000f3"
    // ; let constructor_bytecode = "33600055";
    // let churn_res = cg.churn(inputs, main_bytecode, constructor_bytecode);
    // assert!(churn_res.is_ok());
    // assert_eq!(churn_res.unwrap().bytecode,
    // "336000556101ac806100116000396000f360003560E01c8063a9059cbb1461004857806340c10f19146100de57806370a082311461014e57806318160ddd1461016b578063095ea7b314610177578063dd62ed3e1461018e575b600435336024358160016000526000602001526040600020548082116100d8578190038260016000526000602001526040600020558281906001600052600060200152604060002054018360016000526000602001526040600020556000527fDDF252AD1BE2C89B69C2B068FC378DAA952BA7F163C4A11628F55A4DF523B3EF60206000a3600160005260206000f35b60006000fd5b60005433146100ed5760006000fd5b600435600060243582819060016000526000602001526040600020540183600160005260006020015260406000205580600254016002556000527fDDF252AD1BE2C89B69C2B068FC378DAA952BA7F163C4A11628F55A4DF523B3EF60206000a35b600435600160005260006020015260406000205460005260206000f35b60025460005260206000f35b602435600435336000526000602001526040600020555b60243560043560005260006020015260406000205460005260206000f3"
    // );

    // // Validate the Codegen Artifact
    // assert!(cg.artifact.is_some());
    // assert!(!cg.artifact.clone().unwrap().bytecode.is_empty());
    // assert_eq!(cg.artifact.unwrap().runtime.len(), main_bytecode.len());
}