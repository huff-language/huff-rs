use ethers::types::U256;
use huff_assert::{errors::ErrorKind, utils::inspect};
use huff_lexer::*;
use huff_parser::Parser;
use huff_utils::prelude::*;
use revm::Return;

#[test]
fn test_valid_assertions() {
    let source = r#"
    #define macro TEST() = takes (0) returns (0) {
        0x10        $ [val]
        pop         $ []
    }
    "#;

    let contract = get_contract(source);

    assert_eq!(contract.macros.len(), 1);

    let res = inspect(
        &contract,
        contract.macros.get(0).unwrap(),
        String::default(),
        U256::zero(),
        vec![],
    );

    assert_eq!(res.name, "TEST");
    assert!(res.errors.is_empty());
    assert_eq!(res.reason, Return::Stop);
}

#[test]
fn test_wrong_amount() {
    let source = r#"
    #define macro TEST() = takes (0) returns (0) {
        0x10        $ [val]
        pop         $ [val]
    }
    "#;

    let contract = get_contract(source);

    assert_eq!(contract.macros.len(), 1);

    let res = inspect(
        &contract,
        contract.macros.get(0).unwrap(),
        String::default(),
        U256::zero(),
        vec![],
    );

    assert_eq!(res.name, "TEST");
    assert_eq!(res.errors.len(), 1);

    let err = res.errors.get(0).unwrap();
    assert_eq!(err.kind, ErrorKind::Amount);
    assert_eq!(err.expected, "`[val]`");
    assert_eq!(err.got, "`[]`");

    assert_eq!(res.reason, Return::Stop); // didn't reverted
}

#[test]
fn test_stack_underflow() {
    let source = r#"
    #define macro TEST() = takes (1) returns (0) {
        0x10        $ [val]
        pop pop     $ []
    }
    "#;

    let contract = get_contract(source);

    assert_eq!(contract.macros.len(), 1);

    let res = inspect(
        &contract,
        contract.macros.get(0).unwrap(),
        String::default(),
        U256::zero(),
        vec![],
    );

    assert_eq!(res.name, "TEST");
    assert!(!res.errors.is_empty());
    assert_eq!(res.reason, Return::StackUnderflow);
}

#[test]
fn test_wrong_takes() {
    let source = r#"
    #define macro TEST() = takes (2) returns (0) {
        0x10        $ [val]
        pop         $ []
    }
    "#;

    let contract = get_contract(source);

    assert_eq!(contract.macros.len(), 1);

    let res = inspect(
        &contract,
        contract.macros.get(0).unwrap(),
        String::default(),
        U256::zero(),
        vec![],
    );

    assert_eq!(res.name, "TEST");
    assert_eq!(res.errors.len(), 1);

    let err = res.errors.get(0).unwrap();
    assert_eq!(err.kind, ErrorKind::Takes);
    assert_eq!(err.expected, "`takes(2)`");
    assert_eq!(err.got, "`[]`");

    assert_eq!(res.reason, Return::Stop); // didn't reverted
}

#[test]
fn test_wrong_returns() {
    let source = r#"
    #define macro TEST() = takes (0) returns (0) {
        0x10        $ [val]
    }
    "#;

    let contract = get_contract(source);

    assert_eq!(contract.macros.len(), 1);

    let res = inspect(
        &contract,
        contract.macros.get(0).unwrap(),
        String::default(),
        U256::zero(),
        vec![],
    );

    assert_eq!(res.name, "TEST");
    assert_eq!(res.errors.len(), 1);

    let err = res.errors.get(0).unwrap();
    assert_eq!(err.kind, ErrorKind::Returns);
    assert_eq!(err.expected, "`returns(0)`");
    assert_eq!(err.got, "`[16]`");

    assert_eq!(res.reason, Return::Stop); // didn't reverted
}

#[test]
fn test_reverts() {
    let source = r#"
    #define macro TEST() = takes (0) returns (0) {
        0x00 0x00
        revert
    }
    "#;

    let contract = get_contract(source);

    assert_eq!(contract.macros.len(), 1);

    let res = inspect(
        &contract,
        contract.macros.get(0).unwrap(),
        String::default(),
        U256::zero(),
        vec![],
    );

    assert_eq!(res.name, "TEST");
    assert_eq!(res.errors.len(), 0);

    assert_eq!(res.reason, Return::Revert); // didn't reverted
}

fn get_contract(source: &str) -> Contract {
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    parser.parse().unwrap()
}
