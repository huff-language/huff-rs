use huff_lexer::*;
use huff_utils::prelude::*;
use std::fs;

#[test]
fn lexes_erc20_without_error() {
    let source = fs::read_to_string("../huff-examples/erc20/contracts/ERC20.huff").unwrap();
    let mut lexer = Lexer::new(&source);

    while let Some(token) = lexer.next() {
        println!("{:?}", token.kind);
        assert_ne!(token.kind, TokenKind::Error);
    }
}
