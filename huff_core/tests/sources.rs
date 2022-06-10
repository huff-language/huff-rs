use std::path::PathBuf;

use huff_core::Compiler;
use huff_utils::files::FileSource;

#[test]
fn test_fetch_sources() {
    let file_sources: Vec<FileSource> = Compiler::fetch_sources(&vec![
        PathBuf::from("./examples/ERC20.huff".to_string()),
        PathBuf::from("./examples/utilities/Address.huff".to_string()),
        PathBuf::from("./examples/utilities/HashMap.huff".to_string()),
    ]);
    assert_eq!(file_sources.len(), 3);
    assert_eq!(file_sources[0].path, "./examples/ERC20.huff".to_string());
    assert_eq!(file_sources[1].path, "./examples/utilities/Address.huff".to_string());
    assert_eq!(file_sources[2].path, "./examples/utilities/HashMap.huff".to_string());
}

#[test]
fn test_fetch_invalid_sources() {
    let file_sources: Vec<FileSource> = Compiler::fetch_sources(&vec![
        PathBuf::from("./examples/non_existant.huff".to_string()),
        PathBuf::from("./examples/non_huff.txt".to_string()),
        PathBuf::from("./examples/random/Address.huff".to_string()),
        PathBuf::from("./examples/random/".to_string()),
        PathBuf::from("./examples/utilities/".to_string()),
    ]);
    assert_eq!(file_sources.len(), 0);
}
