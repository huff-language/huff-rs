use huff_codegen::Codegen;
use huff_lexer::*;
use huff_parser::Parser;
use huff_utils::{
    error::CodegenErrorKind,
    prelude::{EVMVersion, FullFileSource, Token},
};

#[test]
fn test_invocation_should_fail() {
    let source: &str = r#"
        #define test MY_TEST() = takes (0) returns (0) {
            0x00 0x01 eq
        }

        #define macro MAIN() = takes (0) returns (0) {
            0x00 calldataload 0xE0 shr
            dup1 __FUNC_SIG(test1) eq test_one jumpi

            test_one:
                MY_TEST()
        }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Parse the AST
    let mut contract = parser.parse().unwrap();

    // Derive storage pointers
    contract.derive_storage_pointers();

    // Instantiate Codegen
    let cg = Codegen::new();

    // The codegen instance should have no artifact
    assert!(cg.artifact.is_none());

    // Have the Codegen create the runtime bytecode. Should throw an error because test
    // invocation is not allowed.
    match Codegen::generate_main_bytecode(&EVMVersion::default(), &contract, None) {
        Ok(_) => panic!("Expected an error"),
        Err(e) => {
            assert_eq!(
                std::mem::discriminant(&e.kind),
                std::mem::discriminant(&CodegenErrorKind::TestInvocation(String::default()))
            );
        }
    }
}
