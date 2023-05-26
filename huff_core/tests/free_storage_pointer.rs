use huff_codegen::Codegen;
use huff_lexer::*;
use huff_parser::Parser;
use huff_utils::{
    prelude::{EVMVersion, FullFileSource},
    token::Token,
};

/// Check that free storage pointers referenced outside of macro bodies
/// are assigned correctly at compilation
#[test]
fn test_set_free_storage_pointers() {
    let source: &str = r#"
        #define constant FREE = FREE_STORAGE_POINTER()

        #define macro TEST_MACRO(slot) = {
            <slot> sload 0x00 mstore 0x20 0x00 return
        }

        #define macro MAIN() = {
            TEST_MACRO(FREE)
        }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Parse AST
    let mut contract = parser.parse().unwrap();

    // Derive storage pointers
    contract.derive_storage_pointers();

    // Assert the Free storage pointer has been set to 0
    let mbytes = Codegen::generate_main_bytecode(&EVMVersion::default(), &contract, None).unwrap();
    assert!(mbytes.starts_with("6000"));
}
