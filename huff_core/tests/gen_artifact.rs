use std::sync::Arc;

use huff_core::Compiler;
use huff_utils::prelude::*;

#[test]
fn test_missing_constructor() {
    let source = r#"
    #define macro MINT() = takes(0) returns (0) {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]
    }

    #define macro MAIN() = takes(0) returns (0) {
        0x00 calldataload 0xE0 shr
        dup1 0x40c10f19 eq mints jumpi

        mints:
            MINT()
    }
    "#;

    // Full source
    let full_source = FileSource {
        source: Some(source.to_string()),
        id: uuid::Uuid::new_v4(),
        path: "".to_string(),
        access: None,
        dependencies: None,
    };

    // Instantiate a new compiler
    let compiler = Compiler::new(Arc::new(vec![]), None, None, false, false);

    // Generate the compile artifact
    let arc_source = Arc::new(full_source);
    match compiler.gen_artifact(Arc::clone(&arc_source)) {
        Ok(artifact) => {
            assert_eq!(artifact.file, arc_source);
            assert_eq!(
                artifact.bytecode,
                "601a8060093d393df360003560e01c806340c10f1914610011575b6004356000602435"
                    .to_string()
            );
        }
        _ => panic!("moose"),
    }
}

#[test]
fn test_missing_constructor_with_inputs() {
    let source = r#"
    #define macro MINT() = takes(0) returns (0) {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]
    }

    #define macro MAIN() = takes(0) returns (0) {
        0x00 calldataload 0xE0 shr
        dup1 0x40c10f19 eq mints jumpi

        mints:
            MINT()
    }
    "#;

    // Full source
    let full_source = FileSource {
        source: Some(source.to_string()),
        id: uuid::Uuid::new_v4(),
        path: "".to_string(),
        access: None,
        dependencies: None,
    };

    // Instantiate a new compiler
    let compiler = Compiler::new(Arc::new(vec![]), None, Some(vec!["0".to_string()]), false, false);

    // Generate the compile artifact
    let arc_source = Arc::new(full_source);
    match compiler.gen_artifact(Arc::clone(&arc_source)) {
        Ok(_) => panic!("moose"),
        Err(e) => {
            assert_eq!(
                e,
                CompilerError::CodegenError(CodegenError {
                    kind: CodegenErrorKind::MissingMacroDefinition("CONSTRUCTOR".to_string()),
                    span: AstSpan(vec![Span { start: 0, end: 0, file: Some(arc_source) }]),
                    token: None
                })
            )
        }
    }
}
