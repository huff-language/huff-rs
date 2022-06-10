/// Tests lexing the Opcodes
use huff_lexer::*;
use huff_utils::{
    evm::{OPCODES, OPCODES_MAP},
    prelude::*,
};

#[test]
fn single_opcode() {
    let source = "push1";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // The first token should be opcode
    let tok = lexer.next().unwrap();
    assert_eq!(
        tok,
        Token::new(
            TokenKind::Opcode(OPCODES_MAP.get("push1").unwrap().to_owned()),
            Span::new(0..source.len())
        )
    );
    assert_eq!(lexer.span, Span::new(0..source.len()));

    // We should have reached EOF now
    assert_eq!(lexer.span.end, source.len());
}

#[test]
fn opcodes() {
    for opcode in OPCODES {
        let opcode = (*opcode).to_owned();
        let source = opcode.clone();
        let mut lexer = Lexer::new(&source);
        assert_eq!(lexer.source, source);

        // The first token should be opcode
        let tok = lexer.next().unwrap();
        assert_eq!(
            tok,
            Token::new(
                TokenKind::Opcode(OPCODES_MAP.get(&opcode).unwrap().to_owned()),
                Span::new(0..opcode.len())
            )
        );
        assert_eq!(lexer.span, Span::new(0..opcode.len()));

        // We should have reached EOF now
        assert_eq!(lexer.span.end, source.len());
        assert!(lexer.next().is_none());
        assert!(lexer.eof);
    }
}
