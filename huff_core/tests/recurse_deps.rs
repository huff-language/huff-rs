use std::path::PathBuf;

use huff_core::Compiler;
use huff_utils::files::FileSource;

#[test]
fn test_recursing_fs_dependencies() {
    let file_sources: Vec<FileSource> =
        Compiler::fetch_sources(&vec![PathBuf::from("./examples/ERC20.huff".to_string())]);
    assert_eq!(file_sources.len(), 1);
    let erc20_file_source = file_sources[0].clone();
    let res = Compiler::recurse_deps(erc20_file_source);
    let full_erc20_file_source = res.unwrap();
    let dependencies = full_erc20_file_source.dependencies.unwrap();
    assert_eq!(dependencies.len(), 4);
    for dep in dependencies {
        assert!(dep.source.is_some());
        assert_eq!(dep.dependencies.unwrap().len(), 0);
    }
}

#[test]
fn test_recursing_external_dependencies() {
    let file_sources: Vec<FileSource> = Compiler::fetch_sources(&vec![PathBuf::from(
        "../huff_lexer/examples/ERC20.huff".to_string(),
    )]);
    assert_eq!(file_sources.len(), 1);
    let erc20_file_source = file_sources[0].clone();
    let res = Compiler::recurse_deps(erc20_file_source);
    let full_erc20_file_source = res.unwrap();
    let dependencies = full_erc20_file_source.dependencies.unwrap();
    assert_eq!(dependencies.len(), 4);
    for dep in dependencies {
        assert!(dep.source.is_some());
        assert_eq!(dep.dependencies.unwrap().len(), 0);
    }
}
