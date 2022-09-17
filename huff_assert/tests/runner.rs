use ethers::types::U256;
use huff_assert::HuffAssert;
use huff_lexer::*;
use huff_parser::Parser;
use huff_tests::types::TestStatus;
use huff_utils::{prelude::*, token::TokenKind::Str};

#[test]
fn test_valid_assertions() {
    let source = r#"
    #define macro TEST() = takes (0) returns (0) {
        0x10        // $ [val]
        pop         $ []
    }
    "#;
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    let contract = parser.parse().unwrap();

    assert_eq!(contract.macros.len(), 1);

    let assert = HuffAssert::new(&contract);

    let res = assert.inspect(contract.macros.get(0).unwrap(), String::default(), U256::zero());

    assert_eq!(res.name, "TEST");
    assert!(res.errors.is_empty());
    assert_eq!(res.status, TestStatus::Success);
}

#[test]
fn test_wrong_takes() {
    let source = r#"
    #define macro TEST() = takes (1) returns (0) {
        0x10        // $ [val]
        pop         $ []
    }
    "#;
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    let contract = parser.parse().unwrap();

    assert_eq!(contract.macros.len(), 1);

    let assert = HuffAssert::new(&contract);

    let res = assert.inspect(contract.macros.get(0).unwrap(), String::default(), U256::zero());

    assert_eq!(res.name, "TEST");
    assert!(!res.errors.is_empty());
    assert_eq!(res.status, TestStatus::Success); // didn't reverted
}
