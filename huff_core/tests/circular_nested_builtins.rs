use huff_codegen::*;
use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

/// This test demonstrates that nested circular builtins break code generation.
#[test]
fn test_circular_nested_builtins() {
    let source = r#"
    #define macro CONSTRUCTOR() = {
        A()
    }

    #define macro A() = {
        pc __codesize(B)
    }

    #define macro B() = {
        pc __codesize(A)
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create constructor bytecode
    match Codegen::generate_constructor_bytecode(&contract, None) {
        Ok((mb, _)) => assert_eq!("586003".to_string(), mb),
        Err(_) => panic!("moose"),
    }
}

#[test]
fn test_obscure_circular_nested_builtins() {
    let source = r#"
    #define macro CONSTRUCTOR() = {
        pc __codesize(A) pc
        A(pc)
    }

    #define macro A(arg) = {
        <arg>
        __codesize(B)
        pc
    }

    #define macro B() = {
        pc
        __codesize(A)
    }
    "#;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create constructor bytecode
    match Codegen::generate_constructor_bytecode(&contract, None) {
        Ok((mb, _)) => assert_eq!("5860045858600358".to_string(), mb),
        Err(_) => panic!("moose"),
    }
}
