/// Tests lexing the Free Storage Pointer Keyword
use huff_lexer::*;
use huff_utils::prelude::*;
use huff_utils::evm::{OPCODES, OPCODES_MAP};

#[test]
fn opcodes() {
    for opcode in OPCODES_MAP.keys() {
        let opcode = (*opcode).to_owned();
        let source = opcode.clone();
        let mut lexer = Lexer::new(&source);
        assert_eq!(lexer.source, source);
    
        // The first token should be opcode
        let tok = lexer.next().unwrap().unwrap();
        assert_eq!(tok, Token::new(TokenKind::Opcode(OPCODES_MAP.get(&opcode).unwrap().to_owned()), Span::new(0..opcode.len())));
        assert_eq!(lexer.span, Span::new(0..opcode.len()));
    
        // We should have reached EOF now
        assert_eq!(lexer.span.end, source.len());
        assert!(lexer.eof);
    }
}
