use std::{path::PathBuf, sync::Arc};

use huff_codegen::Codegen;
use huff_core::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::{file_provider::FileSystemFileProvider, files, prelude::*};

#[test]
fn test_erc20_compile() {
    let file_provider = Arc::new(FileSystemFileProvider {});
    let file_sources: Vec<Arc<FileSource>> = Compiler::fetch_sources(
        vec![PathBuf::from("../huff-examples/erc20/contracts/ERC20.huff".to_string())],
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

    // Full expected bytecode output (generated from huffc)
    let expected_bytecode = "335f5561019980600d3d393df35f3560e01c8063a9059cbb1461004757806340c10f19146100d757806370a082311461014157806318160ddd1461015c578063095ea7b314610166578063dd62ed3e1461017d575b600435336024358160016000526000602001526040600020548082116100d3578190038260016000526000602001526040600020558281906001600052600060200152604060002054018360016000526000602001526040600020555f527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60205fa360015f5260205ff35b5f5ffd5b5f5433146100e3575f5ffd5b6004355f60243582819060016000526000602001526040600020540183600160005260006020015260406000205580600254016002555f527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60205fa35b60043560016000526000602001526040600020545f5260205ff35b6002545f5260205ff35b602435600435336000526000602001526040600020555b6024356004356000526000602001526040600020545f5260205ff3";

    assert_eq!(artifact.bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}
