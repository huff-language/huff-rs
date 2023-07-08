use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn test_alternative_constructor_macro_provided() {
    let source = r#"
    #define macro CONSTRUCTOR() = {
        0x1
    }

    #define macro ALT_CONSTRUCTOR() = {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    let alternative_constructor_label = Some(String::from("ALT_CONSTRUCTOR"));

    // Create constructor bytecode
    match Codegen::generate_constructor_bytecode(
        &EVMVersion::default(),
        &contract,
        alternative_constructor_label,
    ) {
        Ok((mb, _)) => assert_eq!(mb, "6004355f602435".to_string()),
        Err(_) => panic!("moose"),
    }
}
