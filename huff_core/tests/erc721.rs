use std::{path::PathBuf, sync::Arc};

use huff_codegen::Codegen;
use huff_core::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::{file_provider::FileSystemFileProvider, files, prelude::*};

#[test]
fn test_erc721_compile() {
    let file_provider = Arc::new(FileSystemFileProvider {});
    let file_sources: Vec<Arc<FileSource>> = Compiler::fetch_sources(
        vec![PathBuf::from("../huff-examples/erc721/contracts/ERC721.huff".to_string())],
        file_provider.clone(),
    )
    .iter()
    .map(|p| p.clone().unwrap())
    .collect();

    // Recurse file deps + generate flattened source
    let file_source = file_sources.get(0).unwrap();
    let recursed_file_source =
        Compiler::recurse_deps(Arc::clone(file_source), &files::Remapper::new("./"), file_provider)
            .unwrap();
    let flattened = FileSource::fully_flatten(Arc::clone(&recursed_file_source));
    let full_source = FullFileSource {
        source: &flattened.0,
        file: Some(Arc::clone(file_source)),
        spans: flattened.1,
    };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("../huff-examples/erc20/contracts".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    let evm_version = &EVMVersion::default();

    // Create main and constructor bytecode
    let main_bytecode = Codegen::generate_main_bytecode(evm_version, &contract, None).unwrap();
    let (constructor_bytecode, has_custom_bootstrap) =
        Codegen::generate_constructor_bytecode(evm_version, &contract, None).unwrap();

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
    let expected_bytecode = "335f5561038680600d3d393df35f3560e01c8063a9059cbb1461009f57806342842e0e1461019e578063b88d4fde146101a2578063095ea7b314610267578063a22cb465146102f6578063081812fc146102db57806340c10f19146101a657806370a082311461024c5780636352211e1461036b57806306fdde031461033f57806395d89b4114610343578063c87b56dd1461034757806301ffc9a71461034b578063e985e9c51461034f575b6044356024356004358083600160005260006020015260406000205491146100c65761019a565b8033146100ff5733816000526000602001526040600020546100ff5782600260005260006020015260406000205433146100ff5761019a565b600181600360005260006020015260406000205403816003600052600060200152604060002055816003600052600060200152604060002054600101826003600052600060200152604060002055818360016000526000602001526040600020555f8360026000526000602001526040600020557fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60205fa4005b5f5ffd5b5f5ffd5b5f5ffd5b5f5433146101b2575f5ffd5b6024356004355f826001600052600060200152604060002054156101d557610248565b816003600052600060200152604060002054600101826003600052600060200152604060002055818360016000526000602001526040600020555f8360026000526000602001526040600020557fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef5f5fa4005b5f5ffd5b60043560036000526000602001526040600020545f5260205ff35b60243580600160005260006020015260406000205480331433826000526000602001526040600020541761029a576102d7565b60043580836002600052600060200152604060002055907f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9255f5fa4005b5f5ffd5b60043560026000526000602001526040600020545f5260205ff35b60243560043533600052600060200152604060002055600435336024355f527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c315f5fa4005b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b6024356004356000526000602001526040600020545f5260205ff35b60043560016000526000602001526040600020545f5260205ff3";

    assert_eq!(artifact.bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}
