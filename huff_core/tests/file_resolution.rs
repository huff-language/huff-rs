use std::path::PathBuf;

use huff_core::Compiler;
use huff_utils::prelude::{CompilerError, OutputLocation, UnpackError};

#[test]
fn test_get_outputs_no_output() {
    let compiler: Compiler = Compiler::new(vec![], None, None, false);
    let ol: OutputLocation = compiler.get_outputs();
    assert_eq!(ol, OutputLocation::default());
}

#[test]
fn test_get_outputs_with_output() {
    let compiler: Compiler = Compiler::new(vec![], Some("./test_out/".to_string()), None, false);
    let ol: OutputLocation = compiler.get_outputs();
    assert_eq!(ol, OutputLocation("./test_out/".to_string()));
}

#[test]
fn test_transform_paths() {
    let _compiler: Compiler = Compiler::new(vec![], Some("./test_out/".to_string()), None, false);
    let path_bufs: Result<Vec<PathBuf>, CompilerError<'_>> = Compiler::transform_paths(&vec![
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
    let _compiler: Compiler = Compiler::new(vec![], Some("./test_out/".to_string()), None, false);
    let path_bufs: Result<Vec<PathBuf>, CompilerError<'_>> =
        Compiler::transform_paths(&vec!["./ERC20.txt".to_string()]);
    assert!(path_bufs.is_err());
    match path_bufs {
        Err(CompilerError::FileUnpackError(e)) => {
            assert_eq!(e, UnpackError::UnsupportedExtension)
        }
        _ => {
            panic!("moose")
        }
    }
}

#[test]
fn test_transform_paths_no_dir() {
    let _compiler: Compiler = Compiler::new(vec![], Some("./test_out/".to_string()), None, false);
    let path_bufs: Result<Vec<PathBuf>, CompilerError<'_>> =
        Compiler::transform_paths(&vec!["./examples/random_dir/".to_string()]);
    assert!(path_bufs.is_err());
    match path_bufs {
        Err(CompilerError::FileUnpackError(e)) => {
            assert_eq!(e, UnpackError::InvalidDirectory)
        }
        _ => {
            panic!("moose")
        }
    }
}
