use huff_lexer::*;
use huff_utils::prelude::*;
/// Tests lexing the Free Storage Pointer Keyword
use std::ops::Deref;

#[test]
fn free_storage_pointer() {
    let source = "FREE_STORAGE_POINTER() ";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = lexer::LexerNew::new(flattened_source.source.clone());

    // The first token should be the fsp
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::FreeStoragePointer, Span::new(0..21, None)));

    // Eats the whitespace
    let _ = lexer.next();

    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Eof, Span::new(22..22, None)));

    // We should have reached EOF now
    assert!(lexer.eof);
}
