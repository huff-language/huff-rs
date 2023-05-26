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

    // Create main and constructor bytecode using the paris compatible evm version
    let paris_evm = EVMVersion::new(SupportedEVMVersions::Paris);

    let paris_main_bytecode = Codegen::generate_main_bytecode(&paris_evm, &contract, None).unwrap();
    let (paris_constructor_bytecode, paris_has_custom_bootstrap) =
        Codegen::generate_constructor_bytecode(&paris_evm, &contract, None).unwrap();

    // Create main and constructor bytecode using the shanghai compatible evm version
    let shanghai_evm = EVMVersion::new(SupportedEVMVersions::Shanghai);

    let shanghai_main_bytecode =
        Codegen::generate_main_bytecode(&shanghai_evm, &contract, None).unwrap();
    let (shanghai_constructor_bytecode, has_custom_bootstrap) =
        Codegen::generate_constructor_bytecode(&shanghai_evm, &contract, None).unwrap();

    // Churn
    let mut cg = Codegen::new();
    let paris_artifact = cg
        .churn(
            Arc::clone(file_source),
            vec![],
            &paris_main_bytecode,
            &paris_constructor_bytecode,
            paris_has_custom_bootstrap,
        )
        .unwrap();

    // Full expected bytecode output (generated from huffc)
    let expected_paris_bytecode = "336000556103b180600e3d393df360003560e01c8063a9059cbb146100a057806342842e0e146101a3578063b88d4fde146101a9578063095ea7b31461027b578063a22cb46514610310578063081812fc146102f357806340c10f19146101af57806370a082311461025e5780636352211e1461039457806306fdde031461035e57806395d89b4114610364578063c87b56dd1461036a57806301ffc9a714610370578063e985e9c514610376575b6044356024356004358083600160005260006020015260406000205491146100c75761019d565b8033146101005733816000526000602001526040600020546101005782600260005260006020015260406000205433146101005761019d565b6001816003600052600060200152604060002054038160036000526000602001526040600020558160036000526000602001526040600020546001018260036000526000602001526040600020558183600160005260006020015260406000205560008360026000526000602001526040600020557fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206000a4005b60006000fd5b60006000fd5b60006000fd5b60005433146101be5760006000fd5b6024356004356000826001600052600060200152604060002054156101e257610258565b8160036000526000602001526040600020546001018260036000526000602001526040600020558183600160005260006020015260406000205560008360026000526000602001526040600020557fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60006000a4005b60006000fd5b600435600360005260006020015260406000205460005260206000f35b6024358060016000526000602001526040600020548033143382600052600060200152604060002054176102ae576102ed565b60043580836002600052600060200152604060002055907f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560006000a4005b60006000fd5b600435600260005260006020015260406000205460005260206000f35b60243560043533600052600060200152604060002055600435336024356000527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c3160006000a4005b60006000fd5b60006000fd5b60006000fd5b60006000fd5b60006000fd5b60243560043560005260006020015260406000205460005260206000f35b600435600160005260006020015260406000205460005260206000f3";

    assert_eq!(paris_artifact.bytecode.to_lowercase(), expected_paris_bytecode.to_lowercase());

    let shanghai_artifact = cg
        .churn(
            Arc::clone(file_source),
            vec![],
            &shanghai_main_bytecode,
            &shanghai_constructor_bytecode,
            has_custom_bootstrap,
        )
        .unwrap();

    let expected_shanghai_bytecode = "335f5561038680600d3d393df35f3560e01c8063a9059cbb1461009f57806342842e0e1461019e578063b88d4fde146101a2578063095ea7b314610267578063a22cb465146102f6578063081812fc146102db57806340c10f19146101a657806370a082311461024c5780636352211e1461036b57806306fdde031461033f57806395d89b4114610343578063c87b56dd1461034757806301ffc9a71461034b578063e985e9c51461034f575b6044356024356004358083600160005260006020015260406000205491146100c65761019a565b8033146100ff5733816000526000602001526040600020546100ff5782600260005260006020015260406000205433146100ff5761019a565b600181600360005260006020015260406000205403816003600052600060200152604060002055816003600052600060200152604060002054600101826003600052600060200152604060002055818360016000526000602001526040600020555f8360026000526000602001526040600020557fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60205fa4005b5f5ffd5b5f5ffd5b5f5ffd5b5f5433146101b2575f5ffd5b6024356004355f826001600052600060200152604060002054156101d557610248565b816003600052600060200152604060002054600101826003600052600060200152604060002055818360016000526000602001526040600020555f8360026000526000602001526040600020557fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef5f5fa4005b5f5ffd5b60043560036000526000602001526040600020545f5260205ff35b60243580600160005260006020015260406000205480331433826000526000602001526040600020541761029a576102d7565b60043580836002600052600060200152604060002055907f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9255f5fa4005b5f5ffd5b60043560026000526000602001526040600020545f5260205ff35b60243560043533600052600060200152604060002055600435336024355f527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c315f5fa4005b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b5f5ffd5b6024356004356000526000602001526040600020545f5260205ff35b60043560016000526000602001526040600020545f5260205ff3";

    assert_eq!(
        shanghai_artifact.bytecode.to_lowercase(),
        expected_shanghai_bytecode.to_lowercase()
    );
}
