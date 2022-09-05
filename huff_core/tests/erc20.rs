use std::{path::PathBuf, sync::Arc};

use huff_codegen::Codegen;
use huff_core::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::{files, prelude::*};

#[test]
fn test_erc20_compile() {
    let file_sources: Vec<Arc<FileSource>> = Compiler::fetch_sources(vec![PathBuf::from(
        "../huff-examples/erc20/contracts/ERC20.huff".to_string(),
    )])
    .iter()
    .map(|p| p.clone().unwrap())
    .collect();

    // Recurse file deps + generate flattened source
    let file_source = file_sources.get(0).unwrap();
    let recursed_file_source =
        Compiler::recurse_deps(Arc::clone(file_source), &files::Remapper::new("./")).unwrap();
    let flattened = FileSource::fully_flatten(Arc::clone(&recursed_file_source));
    let full_source = FullFileSource {
        source: &flattened.0,
        file: Some(Arc::clone(file_source)),
        spans: flattened.1,
    };
    let lexer = Lexer::new(full_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("../huff-examples/erc20/contracts".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create main and constructor bytecode
    let main_bytecode = Codegen::generate_main_bytecode(&contract).unwrap();
    let constructor_bytecode = Codegen::generate_constructor_bytecode(&contract).unwrap();

    // Churn
    let mut cg = Codegen::new();
    let artifact =
        cg.churn(Arc::clone(file_source), vec![], &main_bytecode, &constructor_bytecode).unwrap();

    // Full expected bytecode output (generated from huffc)
    let expected_bytecode = "336000556101ac80600e3d393df360003560e01c8063a9059cbb1461004857806340c10f19146100de57806370a082311461014e57806318160ddd1461016b578063095ea7b314610177578063dd62ed3e1461018e575b600435336024358160016000526000602001526040600020548082116100d8578190038260016000526000602001526040600020558281906001600052600060200152604060002054018360016000526000602001526040600020556000527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206000a3600160005260206000f35b60006000fd5b60005433146100ed5760006000fd5b600435600060243582819060016000526000602001526040600020540183600160005260006020015260406000205580600254016002556000527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206000a35b600435600160005260006020015260406000205460005260206000f35b60025460005260206000f35b602435600435336000526000602001526040600020555b60243560043560005260006020015260406000205460005260206000f3";

    assert_eq!(artifact.bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}
