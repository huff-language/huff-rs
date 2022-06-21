/// Tests lexing the Free Storage Pointer Keyword
use huff_lexer::*;
use huff_utils::{evm::*, prelude::*, types::*};

#[test]
fn function_context() {
    let source = "#define function test(bytes32) {} returns (address)";
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().collect::<Vec<Token>>();

    // check input
    assert_eq!(tokens.get(4).unwrap().kind, TokenKind::PrimitiveType("bytes32")); // check output
    assert_eq!(tokens.get(tokens.len() - 2).unwrap().kind, TokenKind::PrimitiveType("address"));
}

#[test]
fn event_context() {
    let source = "#define event Transfer(bytes32,address)";
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().collect::<Vec<Token>>();

    assert_eq!(tokens.get(tokens.len() - 4).unwrap().kind, TokenKind::PrimitiveType("bytes32"));
}

// /// Won't parse bytes32 as an ident, but as an opcode
#[test]
fn macro_context() {
    let source = "#define macro TEST() = takes (0) returns (0) {byte}";
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().collect::<Vec<Token>>();
    assert_eq!(tokens.get(tokens.len() - 2).unwrap().kind, TokenKind::Opcode("byte"));
}
