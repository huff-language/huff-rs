use std::sync::Arc;

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
    let flattened_source = FullFileSource { source: SOURCE, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let mut contract = parser.parse().unwrap();
    // Derive storage pointers
    contract.derive_storage_pointers();

    // Instantiate Codegen
    let cg = Codegen::new();

    // The codegen instance should have no artifact
    assert!(cg.artifact.is_none());

    // Have the Codegen create the constructor bytecode
    let (cbytes, custom_bootstrap) =
        Codegen::generate_constructor_bytecode(&EVMVersion::default(), &contract, None).unwrap();
    println!("Constructor Bytecode Result: {cbytes:?}");
    assert_eq!(cbytes, String::from("335f55"));
    assert!(!custom_bootstrap);
}

#[test]
fn compiles_runtime_bytecode() {
    // Lex and Parse the source code
    let flattened_source = FullFileSource { source: SOURCE, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let mut contract = parser.parse().unwrap();
    // Derive storage pointers
    contract.derive_storage_pointers();

    // Instantiate Codegen
    let mut cg = Codegen::new();

    // The codegen instance should have no artifact
    assert!(cg.artifact.is_none());

    // Have the Codegen create the constructor bytecode
    let (cbytes, cbootstrap) =
        Codegen::generate_constructor_bytecode(&EVMVersion::default(), &contract, None).unwrap();
    assert_eq!(cbytes, String::from("335f55"));
    assert!(!cbootstrap);

    let inputs: Vec<ethers_core::abi::Token> = vec![];
    // ERC20 Bytecode
    let main_bytecode =
        "5f3560e01c8063a9059cbb1461004757806340c10f19146100d757806370a082311461014157806318160ddd1461015c578063095ea7b314610166578063dd62ed3e1461017d575b600435336024358160016000526000602001526040600020548082116100d3578190038260016000526000602001526040600020558281906001600052600060200152604060002054018360016000526000602001526040600020555f527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60205fa360015f5260205ff35b5f5ffd5b5f5433146100e3575f5ffd5b6004355f60243582819060016000526000602001526040600020540183600160005260006020015260406000205580600254016002555f527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60205fa35b60043560016000526000602001526040600020545f5260205ff35b6002545f5260205ff35b602435600435336000526000602001526040600020555b6024356004356000526000602001526040600020545f5260205ff3";
    let constructor_bytecode = "335f55";
    let churn_res = cg.churn(
        Arc::new(FileSource::default()),
        inputs,
        main_bytecode,
        constructor_bytecode,
        false,
    );
    assert!(churn_res.is_ok());
    assert_eq!(churn_res.unwrap().bytecode,
        "335f5561019980600d3d393df35f3560e01c8063a9059cbb1461004757806340c10f19146100d757806370a082311461014157806318160ddd1461015c578063095ea7b314610166578063dd62ed3e1461017d575b600435336024358160016000526000602001526040600020548082116100d3578190038260016000526000602001526040600020558281906001600052600060200152604060002054018360016000526000602001526040600020555f527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60205fa360015f5260205ff35b5f5ffd5b5f5433146100e3575f5ffd5b6004355f60243582819060016000526000602001526040600020540183600160005260006020015260406000205580600254016002555f527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60205fa35b60043560016000526000602001526040600020545f5260205ff35b6002545f5260205ff35b602435600435336000526000602001526040600020555b6024356004356000526000602001526040600020545f5260205ff3".to_lowercase()
    );

    // Validate the Codegen Artifact
    assert!(cg.artifact.is_some());
    assert!(!cg.artifact.clone().unwrap().bytecode.is_empty());
    assert_eq!(cg.artifact.unwrap().runtime.len(), main_bytecode.len());
}
