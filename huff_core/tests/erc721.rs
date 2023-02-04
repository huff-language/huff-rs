use std::{path::PathBuf, sync::Arc};

use huff_codegen::Codegen;
use huff_core::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::{files, prelude::*};

#[test]
fn test_erc721_compile() {
    let file_sources: Vec<Arc<FileSource>> = Compiler::fetch_sources(vec![PathBuf::from(
        "../huff-examples/erc721/contracts/ERC721.huff".to_string(),
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
    let main_bytecode = Codegen::generate_main_bytecode(&contract, None).unwrap();
    let (constructor_bytecode, has_custom_bootstrap) =
        Codegen::generate_constructor_bytecode(&contract, None).unwrap();

    // Churn
    let mut cg = Codegen::new();
    let artifact = cg
        .churn(
            Arc::clone(file_source),
            vec![],
            &main_bytecode,
            &constructor_bytecode,
            has_custom_bootstrap,
        )
        .unwrap();

    // Full expected bytecode output (different from huffc since our storage pointer derivation is
    // depth first)
    let expected_bytecode = "336000556103b180600e3d393df360003560e01c8063a9059cbb146100a057806342842e0e146101a3578063b88d4fde146101a9578063095ea7b31461027b578063a22cb46514610310578063081812fc146102f357806340c10f19146101af57806370a082311461025e5780636352211e1461039457806306fdde031461035e57806395d89b4114610364578063c87b56dd1461036a57806301ffc9a714610370578063e985e9c514610376575b6044356024356004358083600160005260006020015260406000205491146100c75761019d565b8033146101005733816000526000602001526040600020546101005782600260005260006020015260406000205433146101005761019d565b6001816003600052600060200152604060002054038160036000526000602001526040600020558160036000526000602001526040600020546001018260036000526000602001526040600020558183600160005260006020015260406000205560008360026000526000602001526040600020557fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206000a4005b60006000fd5b60006000fd5b60006000fd5b60005433146101be5760006000fd5b6024356004356000826001600052600060200152604060002054156101e257610258565b8160036000526000602001526040600020546001018260036000526000602001526040600020558183600160005260006020015260406000205560008360026000526000602001526040600020557fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60006000a4005b60006000fd5b600435600360005260006020015260406000205460005260206000f35b6024358060016000526000602001526040600020548033143382600052600060200152604060002054176102ae576102ed565b60043580836002600052600060200152604060002055907f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560006000a4005b60006000fd5b600435600260005260006020015260406000205460005260206000f35b60243560043533600052600060200152604060002055600435336024356000527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c3160006000a4005b60006000fd5b60006000fd5b60006000fd5b60006000fd5b60006000fd5b60243560043560005260006020015260406000205460005260206000f35b600435600160005260006020015260406000205460005260206000f3";

    assert_eq!(artifact.bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}
