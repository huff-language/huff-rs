use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn test_circular_large_constructors() {
    let source = r#"
    #define macro CONSTRUCTOR() = {
        __codesize(CONSTRUCTOR)
        FILLER_MACRO()
    }

    // 254 program counters where the codesize should push us over the word limit
    #define macro FILLER_MACRO() = {
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc
    }

    #define macro MAIN() = {
        0x00
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create constructor bytecode
    match Codegen::generate_constructor_bytecode(&EVMVersion::default(),&contract, None) {
        Ok((mb, _)) => assert_eq!("60ff58585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858".to_string(), mb), 
        Err(_) => panic!("moose"),
    }
}

#[test]
fn test_circular_constructor_at_word_boundry() {
    let source = r#"
    #define macro CONSTRUCTOR() = {
        __codesize(CONSTRUCTOR)
        FILLER_MACRO()
        __codesize(CONSTRUCTOR)
    }

    // 254 program counters where the codesize should push us over the word limit
    #define macro FILLER_MACRO() = {
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc pc
        pc pc pc pc pc pc pc pc pc pc pc pc pc
    }

    #define macro MAIN() = {
        0x00
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create constructor bytecode
    match Codegen::generate_constructor_bytecode(&EVMVersion::default(),&contract, None) {
        Ok((mb, _)) => assert_eq!("61010358585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858585858610103".to_string(), mb), 
        Err(_) => panic!("moose"),
    }
}

#[test]
fn test_double_circular_constructor_multiple_macro_invocations() {
    let source = r#"
    #define macro CONSTRUCTOR() = {
        __codesize(CONSTRUCTOR)
        CODESIZE_CONSTRUCTOR()
        __codesize(CONSTRUCTOR)
    }

    // Tries to get the codesize of the constructor macro
    #define macro CODESIZE_CONSTRUCTOR() = {
        pc __codesize(CONSTRUCTOR)
    }

    #define macro MAIN() = {
        0x00
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create constructor bytecode
    match Codegen::generate_constructor_bytecode(&EVMVersion::default(), &contract, None) {
        Ok((mb, _)) => assert_eq!("60075860076007".to_string(), mb),
        Err(_) => panic!("moose"),
    }
}

#[test]
fn test_double_circular_constructor_nested_macro_invocations() {
    let source = r#"
    #define macro CONSTRUCTOR() = {
        __codesize(CONSTRUCTOR)
        CODESIZED_CODESIZE()
        __codesize(CONSTRUCTOR)
    }

    #define macro CODESIZED_CODESIZE() = {
        CODESIZE_CONSTRUCTOR()
        pc __codesize(CODESIZE_CONSTRUCTOR)
    }

    #define macro CODESIZE_CONSTRUCTOR() = {
        pc __codesize(CONSTRUCTOR)
    }

    #define macro MAIN() = {
        0x00
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create constructor bytecode
    match Codegen::generate_constructor_bytecode(&EVMVersion::default(), &contract, None) {
        Ok((mb, _)) => assert_eq!("600a58600a586003600a".to_string(), mb),
        Err(_) => panic!("moose"),
    }
}

// TODO: there could be some arg invocations that mess with codesize circulation
