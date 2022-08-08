use std::{str::FromStr, sync::Arc};

use huff_core::Compiler;
use huff_utils::prelude::*;

// TODO: fix this test by writing broken inputs to a temp file?
// #[test]
// fn test_parses_foundry_aliased_imports() {
//     println!("In env: {:?}", std::env::current_dir().unwrap());
//     std::env::set_current_dir("../");
//     println!("In env: {:?}", std::env::current_dir().unwrap());
//     let import_bufs =
//         vec![std::path::PathBuf::from_str("examples/erc20/contracts/ERC20.huff").unwrap()];
//     let potentials = Compiler::fetch_sources(import_bufs)
//         .into_iter()
//         .collect::<Vec<Result<Arc<FileSource>, CompilerError<'_>>>>();
//     for import in potentials {
//         import.unwrap();
//     }
// }

#[test]
#[should_panic]
fn test_invalid_imports_break() {
    let import_bufs =
        vec![std::path::PathBuf::from_str("unaliased/erc20/contracts/ERC20.huff").unwrap()];
    let potentials = Compiler::fetch_sources(import_bufs)
        .into_iter()
        .collect::<Vec<Result<Arc<FileSource>, CompilerError<'_>>>>();
    for import in potentials {
        import.unwrap();
    }
}
