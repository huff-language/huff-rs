/// Tests lexing the Free Storage Pointer Keyword
use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn free_storage_pointer() {
    let source = "FREE_STORAGE_POINTER() ";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // The first token should be the fsp
    let tok = lexer.next().unwrap();
    assert_eq!(tok, Token::new(TokenKind::FreeStoragePointer, Span::new(0..22)));
    assert_eq!(lexer.span, Span::new(0..22));

    // Eats the whitespace
    let _ = lexer.next();

    // We should have reached EOF now
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}
