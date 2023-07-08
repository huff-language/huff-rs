use criterion::{criterion_group, criterion_main, Criterion};
use huff_codegen::*;
use huff_core::Compiler;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::{file_provider::FileSystemFileProvider, files, prelude::*};
use std::{path::PathBuf, sync::Arc};

fn lex_erc20_from_source_benchmark(c: &mut Criterion) {
    let file_provider = Arc::new(FileSystemFileProvider::new());
    let file_sources: Vec<Arc<FileSource>> = Compiler::fetch_sources(
        vec![PathBuf::from("../huff-examples/erc20/contracts/ERC20.huff".to_string())],
        file_provider.clone(),
    )
    .into_iter()
    .map(|p| p.unwrap())
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

    // Isolate lexing to benchmark
    c.bench_function("Lexer: ERC-20", |b| {
        b.iter(|| {
            let lexer = Lexer::new(full_source.source);
            let _ = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
        })
    });
}

fn parse_erc20_benchmark(c: &mut Criterion) {
    let file_provider = Arc::new(FileSystemFileProvider::new());
    let file_sources: Vec<Arc<FileSource>> = Compiler::fetch_sources(
        vec![PathBuf::from("../huff-examples/erc20/contracts/ERC20.huff".to_string())],
        file_provider.clone(),
    )
    .into_iter()
    .map(|p| p.unwrap())
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
    let tokens = Box::new(lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>());

    // Isolate parsing to benchmark
    c.bench_function("Parser: ERC-20", |b| {
        b.iter(|| {
            let mut parser =
                Parser::new(*tokens.clone(), Some("../huff-examples/erc20/contracts".to_string()));
            let mut contract = parser.parse().unwrap();
            contract.derive_storage_pointers();
        })
    });
}

fn codegen_erc20_benchmark(c: &mut Criterion) {
    let file_provider = Arc::new(FileSystemFileProvider::new());
    let file_sources: Vec<Arc<FileSource>> = Compiler::fetch_sources(
        vec![PathBuf::from("../huff-examples/erc20/contracts/ERC20.huff".to_string())],
        file_provider.clone(),
    )
    .into_iter()
    .map(|p| p.unwrap())
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

    // Isolate codegen to benchmark
    c.bench_function("Codegen: ERC-20", |b| b.iter(|| {
        // Create main and constructor bytecode
        let main_bytecode = Codegen::generate_main_bytecode(evm_version,&contract, None).unwrap();
        let (constructor_bytecode, has_custom_bootstrap) = Codegen::generate_constructor_bytecode(evm_version,&contract, None).unwrap();

        // Churn
        let mut cg = Codegen::new();
        let artifact =
            cg.churn(file_source.clone(), vec![], &main_bytecode, &constructor_bytecode, has_custom_bootstrap).unwrap();

        // Full expected bytecode output (generated from huffc)
        let expected_bytecode = "336000556101ac80600e3d393df360003560e01c8063a9059cbb1461004857806340c10f19146100de57806370a082311461014e57806318160ddd1461016b578063095ea7b314610177578063dd62ed3e1461018e575b600435336024358160016000526000602001526040600020548082116100d8578190038260016000526000602001526040600020558281906001600052600060200152604060002054018360016000526000602001526040600020556000527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206000a3600160005260206000f35b60006000fd5b60005433146100ed5760006000fd5b600435600060243582819060016000526000602001526040600020540183600160005260006020015260406000205580600254016002556000527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206000a35b600435600160005260006020015260406000205460005260206000f35b60025460005260206000f35b602435600435336000526000602001526040600020555b60243560043560005260006020015260406000205460005260206000f3";

        assert_eq!(artifact.bytecode.to_lowercase(), expected_bytecode.to_lowercase());
    }));
}

fn erc20_compilation_benchmark(c: &mut Criterion) {
    c.bench_function("Full ERC-20 compilation", |b| b.iter(|| {
        let file_provider = Arc::new(FileSystemFileProvider::new());
        let file_sources: Vec<Arc<FileSource>> = Compiler::fetch_sources(vec![PathBuf::from(
            "../huff-examples/erc20/contracts/ERC20.huff".to_string(),
        )], file_provider.clone())
            .into_iter()
            .map(|p| p.unwrap())
            .collect();

        // Recurse file deps + generate flattened source
        let file_source = file_sources.get(0).unwrap();
        let recursed_file_source = Compiler::recurse_deps(
            Arc::clone(file_source),
            &files::Remapper::new("./"),
            file_provider
        ).unwrap();
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
        let main_bytecode = Codegen::generate_main_bytecode(evm_version,&contract, None).unwrap();
        let (constructor_bytecode, has_custom_bootstrap) = Codegen::generate_constructor_bytecode(evm_version, &contract, None).unwrap();

        // Churn
        let mut cg = Codegen::new();
        let artifact =
            cg.churn(file_source.clone(), vec![], &main_bytecode, &constructor_bytecode, has_custom_bootstrap).unwrap();

        // Full expected bytecode output (generated from huffc)
        let expected_bytecode = "336000556101ac80600e3d393df360003560e01c8063a9059cbb1461004857806340c10f19146100de57806370a082311461014e57806318160ddd1461016b578063095ea7b314610177578063dd62ed3e1461018e575b600435336024358160016000526000602001526040600020548082116100d8578190038260016000526000602001526040600020558281906001600052600060200152604060002054018360016000526000602001526040600020556000527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206000a3600160005260206000f35b60006000fd5b60005433146100ed5760006000fd5b600435600060243582819060016000526000602001526040600020540183600160005260006020015260406000205580600254016002556000527fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206000a35b600435600160005260006020015260406000205460005260206000f35b60025460005260206000f35b602435600435336000526000602001526040600020555b60243560043560005260006020015260406000205460005260206000f3";

        assert_eq!(artifact.bytecode.to_lowercase(), expected_bytecode.to_lowercase());
    }));
}

fn erc721_compilation_benchmark(c: &mut Criterion) {
    c.bench_function("Full ERC-721 compilation", |b| b.iter(|| {
        let file_provider = Arc::new(FileSystemFileProvider::new());
        let file_sources: Vec<Arc<FileSource>> = Compiler::fetch_sources(vec![PathBuf::from(
            "../huff-examples/erc721/contracts/ERC721.huff".to_string(),
        )], file_provider.clone())
            .into_iter()
            .map(|p| p.unwrap())
            .collect();

        // Recurse file deps + generate flattened source
        let file_source = file_sources.get(0).unwrap();
        let recursed_file_source = Compiler::recurse_deps(
            Arc::clone(file_source),
            &files::Remapper::new("./"),
            file_provider
        ).unwrap();
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
        let (constructor_bytecode, has_custom_bootstrap) = Codegen::generate_constructor_bytecode(evm_version,&contract, None).unwrap();

        // Churn
        let mut cg = Codegen::new();
        let artifact =
            cg.churn(file_source.clone(), vec![], &main_bytecode, &constructor_bytecode, has_custom_bootstrap).unwrap();

        // Full expected bytecode output (generated from huffc)
        let expected_bytecode = "336000556103b180600e3d393df360003560e01c8063a9059cbb146100a057806342842e0e146101a3578063b88d4fde146101a9578063095ea7b31461027b578063a22cb46514610310578063081812fc146102f357806340c10f19146101af57806370a082311461025e5780636352211e1461039457806306fdde031461035e57806395d89b4114610364578063c87b56dd1461036a57806301ffc9a714610370578063e985e9c514610376575b6044356024356004358083600160005260006020015260406000205491146100c75761019d565b8033146101005733816000526000602001526040600020546101005782600260005260006020015260406000205433146101005761019d565b6001816003600052600060200152604060002054038160036000526000602001526040600020558160036000526000602001526040600020546001018260036000526000602001526040600020558183600160005260006020015260406000205560008360026000526000602001526040600020557fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60206000a4005b60006000fd5b60006000fd5b60006000fd5b60005433146101be5760006000fd5b6024356004356000826001600052600060200152604060002054156101e257610258565b8160036000526000602001526040600020546001018260036000526000602001526040600020558183600160005260006020015260406000205560008360026000526000602001526040600020557fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef60006000a4005b60006000fd5b600435600360005260006020015260406000205460005260206000f35b6024358060016000526000602001526040600020548033143382600052600060200152604060002054176102ae576102ed565b60043580836002600052600060200152604060002055907f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560006000a4005b60006000fd5b600435600260005260006020015260406000205460005260206000f35b60243560043533600052600060200152604060002055600435336024356000527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c3160006000a4005b60006000fd5b60006000fd5b60006000fd5b60006000fd5b60006000fd5b60243560043560005260006020015260406000205460005260206000f35b600435600160005260006020015260406000205460005260206000f3";

        assert_eq!(artifact.bytecode.to_lowercase(), expected_bytecode.to_lowercase());
    }));
}

criterion_group!(
    benches,
    lex_erc20_from_source_benchmark,
    parse_erc20_benchmark,
    codegen_erc20_benchmark,
    erc20_compilation_benchmark,
    erc721_compilation_benchmark
);
criterion_main!(benches);
