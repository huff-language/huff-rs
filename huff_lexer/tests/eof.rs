use huff_lexer::*;
use huff_utils::prelude::*;
use std::ops::Deref;

#[test]
fn end_of_file() {
    let source = " ";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source.clone());
    assert_eq!(lexer.source, flattened_source);

    // Eats the whitespace
    let _ = lexer.next();

    // Get an EOF token
    let tok = lexer.next();
    let tok = tok.unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Eof, Span::new(1..1, None)));
    assert_eq!(lexer.current_span().deref(), &Span::new(1..1, None));

    // We should have reached EOF now
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
    assert!(lexer.next().is_none());
}
