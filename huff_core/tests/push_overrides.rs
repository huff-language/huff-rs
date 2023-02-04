use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn test_gracefully_pads_push_override() {
    // Create the raw source
    const OVERRIDEN_PUSH: &str = r#"
        #define macro CONSTRUCTOR() = {
            push32 0x234
        }
        #define macro MAIN() = {}
    "#;

    // Lex and Parse the source code
    let flattened_source = FullFileSource { source: OVERRIDEN_PUSH, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let mut contract = parser.parse().unwrap();
    // Derive storage pointers
    contract.derive_storage_pointers();

    // Instantiate Codegen
    let cg = Codegen::new();

    // The codegen instance should have no artifact
    assert!(cg.artifact.is_none());

    // Have the Codegen create the constructor bytecode
    let (cbytes, has_custom_bootstrap) =
        Codegen::generate_constructor_bytecode(&contract, None).unwrap();
    assert_eq!(
        cbytes,
        String::from("7f0000000000000000000000000000000000000000000000000000000000000234")
    );
    assert!(!has_custom_bootstrap);
}

#[test]
fn test_constructs_exact_push_override() {
    // Create the raw source
    const OVERRIDEN_PUSH: &str = r#"
        #define macro CONSTRUCTOR() = {
            push1 0x34
        }
        #define macro MAIN() = {}
    "#;

    // Lex and Parse the source code
    let flattened_source = FullFileSource { source: OVERRIDEN_PUSH, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Grab the first macro
    let mut contract = parser.parse().unwrap();
    // Derive storage pointers
    contract.derive_storage_pointers();

    // Instantiate Codegen
    let cg = Codegen::new();

    // The codegen instance should have no artifact
    assert!(cg.artifact.is_none());

    // Have the Codegen create the constructor bytecode
    let (cbytes, has_custom_bootstrap) =
        Codegen::generate_constructor_bytecode(&contract, None).unwrap();
    assert_eq!(cbytes, String::from("6034"));
    assert!(!has_custom_bootstrap);
}

#[test]
#[should_panic]
fn test_fails_on_push_underflow() {
    const OVERRIDEN_PUSH: &str = r#"
        #define macro CONSTRUCTOR() = {
            push1 0x0234
        }
        #define macro MAIN() = {}
    "#;

    let flattened_source = FullFileSource { source: OVERRIDEN_PUSH, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    parser.parse().unwrap();
}
