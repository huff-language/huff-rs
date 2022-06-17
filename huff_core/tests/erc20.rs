use std::path::PathBuf;

use huff_codegen::Codegen;
use huff_core::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[ignore]
#[test]
fn test_erc20_compile() {
    let file_sources: Vec<FileSource> = Compiler::fetch_sources(&vec![PathBuf::from(
        "../huff-examples/erc20/contracts/ERC20.huff".to_string(),
    )]);

    // Recurse file deps + generate flattened source
    let file_source = file_sources.get(0).unwrap();
    let recursed_file_source = Compiler::recurse_deps(file_source.clone()).unwrap();
    let flattened = recursed_file_source.fully_flatten();
    let full_source = FullFileSource {
        source: &flattened.0,
        file: Some(file_source.clone()),
        spans: flattened.1,
    };
    let lexer = Lexer::new(full_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("../huff-examples/erc20/contracts".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create main and constructor bytecode
    let main_bytecode = Codegen::roll(Some(contract.clone())).unwrap();
    let constructor_bytecode = Codegen::construct(Some(contract.clone())).unwrap();

    // Churn
    let mut cg = Codegen::new();
    let artifact =
        cg.churn(file_source.clone(), vec![], &main_bytecode, &constructor_bytecode).unwrap();

    // Full expected bytecode output (generated from huffc)
    let expected_bytecode = "336000556101ac806100116000396000f360003560E01c8063a9059cbb1461004857806340c10f19146100de57806370a082311461014e57806318160ddd1461016b578063095ea7b314610177578063dd62ed3e1461018e575b600435336024358160016000526000602001526040600020548082116100d8578190038260016000526000602001526040600020558281906001600052600060200152604060002054018360016000526000602001526040600020556000527fDDF252AD1BE2C89B69C2B068FC378DAA952BA7F163C4A11628F55A4DF523B3EF60206000a3600160005260206000f35b60006000fd5b60005433146100ed5760006000fd5b600435600060243582819060016000526000602001526040600020540183600160005260006020015260406000205580600254016002556000527fDDF252AD1BE2C89B69C2B068FC378DAA952BA7F163C4A11628F55A4DF523B3EF60206000a35b600435600160005260006020015260406000205460005260206000f35b60025460005260206000f35b602435600435336000526000602001526040600020555b60243560043560005260006020015260406000205460005260206000f3";
    let _current_bytecode = "336003556101a9806100116000396000f360003560e01c8063a9059cbb1461004857806340c10f19146100db57806370a082311461014b57806318160ddd14610168578063095ea7b314610174578063dd62ed3e1461018b575b60043533602435816000600052600060200152604060002054808211578190038260006000526000602001526040600020558281906000600052600060200152604060002054018360006000526000602001526040600020556000527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206000a3600160005260206000f35b60006000fd5b60035433146100ea5760006000fd5b600435600060243582819060006000526000602001526040600020540183600060005260006020015260406000205580600254016002556000527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206000a35b600435600060005260006020015260406000205460005260206000f35b60025460005260206000f35b602435600435336000526000602001526040600020555b60243560043560005260006020015260406000205460005260206000f3";

    println!("Expected bytecode: {}", expected_bytecode.to_lowercase());
    println!("Current bytecode: {}", artifact.bytecode.to_lowercase());

    // TODO: Check the bytecode
    // assert_eq!(artifact.bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}