/// Tests lexing the Opcodes
use huff_lexer::*;
use huff_utils::{evm::OPCODES, prelude::*};

#[test]
fn single_opcode() {
    let source = "address { address }";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // First token is `address` as Identifier (out of a scope)
    let tok = lexer.next().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Ident("address"), Span::new(0..7)));

    // Second token is Open Brace
    let _ = lexer.next().unwrap();

    // The third token should be opcode `address` (inside a scope)
    let tok = lexer.next().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Opcode("address"), Span::new(10..17)));

    // Last token is Closing Brace
    let _ = lexer.next().unwrap();

    // We should have reached EOF now
    assert_eq!(lexer.span.end, source.len());
}

#[test]
fn opcodes() {
    for opcode in OPCODES {
        let opcode = (*opcode).to_owned();
        // Opcode inside a scope
        let source = format!("{{ {} }}", opcode); // { opcode_name }

        let mut lexer = Lexer::new(&source);
        assert_eq!(lexer.source, source);

        // First token is Opening Brace
        let _ = lexer.next().unwrap();
        // The second token should be opcode
        let tok = lexer.next().unwrap();
        assert_eq!(tok, Token::new(TokenKind::Opcode(&opcode), Span::new(2..2 + opcode.len())));

        // Last token is Closing Brace
        assert_eq!(lexer.span, Span::new(2..2 + opcode.len()));
        let _ = lexer.next().unwrap();
        // We should have reached EOF now
        assert_eq!(lexer.span.end, source.len());
        assert!(lexer.next().is_none());
        assert!(lexer.eof);
    }
}
