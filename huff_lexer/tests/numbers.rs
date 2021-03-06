use huff_lexer::*;
use huff_utils::prelude::*;
use std::ops::Deref;

#[test]
fn lexes_zero_prefixed_numbers() {
    let source = "00";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    // The first and only token should be lexed as 0
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Num(0), Span::new(0..2, None)));
    assert_eq!(lexer.current_span().deref(), &Span::new(0..2, None));

    // We covered the whole source
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

#[test]
fn lexes_large_numbers() {
    let source = &format!("{}", usize::MAX);
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    // The first and only token should be lexed
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Num(usize::MAX), Span::new(0..source.len(), None)));
    assert_eq!(lexer.current_span().deref(), &Span::new(0..source.len(), None));

    // We covered the whole source
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}
