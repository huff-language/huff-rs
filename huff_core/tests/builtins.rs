use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

const SOURCE: &str = r#"
    #define constant OWNER_POINTER = FREE_STORAGE_POINTER()

    #define macro OWNABLE() = takes (0) returns (0) {
        caller [OWNER_POINTER] sstore
    }

    #define macro BUILTIN_TEST() = takes(0) returns(1) {
        __codesize(OWNABLE)
    }

    #define macro CONSTRUCTOR() = takes(0) returns (0) {
        BUILTIN_TEST()
    }
"#;

#[test]
fn test_codesize_builtin() {
    // Parse tokens
    let flattened_source = FullFileSource { source: SOURCE, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
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

    // Have the Codegen create the constructor bytecode
    let cbytes = Codegen::generate_constructor_bytecode(&contract).unwrap();
    println!("Constructor Bytecode Result: {:?}", cbytes);
    assert_eq!(cbytes, String::from("6004"));
}
