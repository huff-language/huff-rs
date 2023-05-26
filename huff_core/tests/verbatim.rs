use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn test_verbatim() {
    let source = r#"
    #define macro MAIN() = takes(0) returns(0) {
        __VERBATIM(0x1234567890abcdef)
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Get main bytecode with verbatim
    match Codegen::generate_main_bytecode(&EVMVersion::default(), &contract, None) {
        Ok(mb) => assert_eq!(mb, "1234567890abcdef".to_string()),
        Err(_) => panic!("moose"),
    }
}

#[test]
fn test_verbatim_invalid_hex() {
    let source = r#"
    #define macro MAIN() = takes(0) returns(0) {
        __VERBATIM("ggggggg")
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Expect failure to generate bytecode with verbatim
    assert!(Codegen::generate_main_bytecode(&EVMVersion::default(), &contract, None).is_err());
}
