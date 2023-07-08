use std::{path::PathBuf, sync::Arc};

use huff_core::Compiler;
use huff_utils::{
    file_provider::{FileProvider, FileSystemFileProvider},
    prelude::{CompilerError, EVMVersion, OutputLocation, UnpackError},
};

#[test]
fn test_get_outputs_no_output() {
    let evm_version = EVMVersion::default();
    let compiler: Compiler =
        Compiler::new(&evm_version, Arc::new(vec![]), None, None, None, None, None, false, false);
    let ol: OutputLocation = compiler.get_outputs();
    assert_eq!(ol, OutputLocation::default());
}

#[test]
fn test_get_outputs_with_output() {
    let evm_version = EVMVersion::default();
    let compiler: Compiler = Compiler::new(
        &evm_version,
        Arc::new(vec![]),
        Some("./test_out/".to_string()),
        None,
        None,
        None,
        None,
        false,
        false,
    );
    let ol: OutputLocation = compiler.get_outputs();
    assert_eq!(ol, OutputLocation("./test_out/".to_string()));
}

#[test]
fn test_transform_paths() {
    let file_provider = FileSystemFileProvider {};
    let path_bufs: Result<Vec<PathBuf>, CompilerError> = file_provider.transform_paths(&[
        "../huff-examples/erc20/contracts/ERC20.huff".to_string(),
        "../huff-examples/erc20/contracts/utils/".to_string(),
    ]);
    assert!(path_bufs.is_ok());
    match path_bufs {
        Ok(bufs) => {
            assert_eq!(bufs.len(), 5);
            assert!(bufs.contains(&PathBuf::from(
                "../huff-examples/erc20/contracts/ERC20.huff".to_string()
            )));
            assert!(bufs.contains(&PathBuf::from(
                "../huff-examples/erc20/contracts/utils/Address.huff".to_string()
            )));
            assert!(bufs.contains(&PathBuf::from(
                "../huff-examples/erc20/contracts/utils/HashMap.huff".to_string()
            )));
            assert!(bufs.contains(&PathBuf::from(
                "../huff-examples/erc20/contracts/utils/Ownable.huff".to_string()
            )));
            assert!(bufs.contains(&PathBuf::from(
                "../huff-examples/erc20/contracts/utils/Utils.huff".to_string()
            )));
        }
        Err(_) => {
            panic!("moose")
        }
    }
}

#[test]
fn test_transform_paths_non_huff() {
    let file_provider = FileSystemFileProvider {};
    let path_bufs: Result<Vec<PathBuf>, CompilerError> =
        file_provider.transform_paths(&["./ERC20.txt".to_string()]);
    assert!(path_bufs.is_err());
    match path_bufs {
        Err(CompilerError::FileUnpackError(e)) => {
            assert_eq!(e, UnpackError::UnsupportedExtension("./ERC20.txt".to_string()))
        }
        _ => {
            panic!("moose")
        }
    }
}

#[test]
fn test_transform_paths_no_dir() {
    let file_provider = FileSystemFileProvider {};
    let path_bufs: Result<Vec<PathBuf>, CompilerError> =
        file_provider.transform_paths(&["./examples/random_dir/".to_string()]);
    assert!(path_bufs.is_err());
    match path_bufs {
        Err(CompilerError::FileUnpackError(e)) => {
            assert_eq!(e, UnpackError::InvalidDirectory("./examples/random_dir/".to_string()))
        }
        _ => {
            panic!("moose")
        }
    }
}
