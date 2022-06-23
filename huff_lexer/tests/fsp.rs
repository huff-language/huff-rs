use huff_lexer::*;
use huff_utils::prelude::*;
/// Tests lexing the Free Storage Pointer Keyword
use std::ops::Deref;

#[test]
fn free_storage_pointer() {
    let source = "FREE_STORAGE_POINTER() ";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source.clone());
    assert_eq!(lexer.source, flattened_source);

    // The first token should be the fsp
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::FreeStoragePointer, Span::new(0..22, None)));
    assert_eq!(lexer.current_span().deref(), &Span::new(0..22, None));

    // Eats the whitespace
    let _ = lexer.next();

    // We should have reached EOF now
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}
