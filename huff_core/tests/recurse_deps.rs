use std::{path::PathBuf, sync::Arc};

use huff_core::Compiler;
use huff_utils::{file_provider::FileSystemFileProvider, files};

#[test]
fn test_recursing_fs_dependencies() {
    let file_provider = Arc::new(FileSystemFileProvider {});
    let file_sources: Vec<Arc<files::FileSource>> = Compiler::fetch_sources(
        vec![PathBuf::from("../huff-examples/erc20/contracts/ERC20.huff".to_string())],
        file_provider.clone(),
    )
    .iter()
    .map(|p| p.clone().unwrap())
    .collect();
    assert_eq!(file_sources.len(), 1);
    let erc20_file_source = file_sources[0].clone();
    let res = Compiler::recurse_deps(
        Arc::clone(&erc20_file_source),
        &files::Remapper::new("./"),
        file_provider,
    );
    let full_erc20_file_source = res.unwrap();
    let dependencies = full_erc20_file_source.dependencies.as_ref().unwrap();
    assert_eq!(dependencies.len(), 4);
    for dep in dependencies {
        assert!(dep.source.is_some());
        assert_eq!(dep.dependencies.as_ref().unwrap().len(), 0);
    }
}

#[test]
fn test_recursing_external_dependencies() {
    let file_provider = Arc::new(FileSystemFileProvider {});
    let file_sources: Vec<Arc<files::FileSource>> = Compiler::fetch_sources(
        vec![PathBuf::from("../huff-examples/erc20/contracts/ERC20.huff".to_string())],
        file_provider.clone(),
    )
    .iter()
    .map(|p| p.clone().unwrap())
    .collect();
    assert_eq!(file_sources.len(), 1);
    let erc20_file_source = file_sources[0].clone();
    let res = Compiler::recurse_deps(
        Arc::clone(&erc20_file_source),
        &files::Remapper::new("./"),
        file_provider,
    );
    let full_erc20_file_source = res.unwrap();
    let dependencies = full_erc20_file_source.dependencies.as_ref().unwrap();
    assert_eq!(dependencies.len(), 4);
    for dep in dependencies {
        assert!(dep.source.is_some());
        assert_eq!(dep.dependencies.as_ref().unwrap().len(), 0);
    }
}
