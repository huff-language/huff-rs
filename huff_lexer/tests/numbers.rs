use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn lexes_zero_prefixed_numbers() {
    let source = "00";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // The first and only token should be lexed as 0
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Num(0), Span::new(0..2)));
    assert_eq!(lexer.span, Span::new(0..2));

    // We covered the whole source
    lexer.next();
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}

#[test]
fn lexes_large_numbers() {
    let source = format!("{}", usize::MAX);
    let mut lexer = Lexer::new(&source);
    assert_eq!(lexer.source, source);

    // The first and only token should be lexed
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Num(usize::MAX), Span::new(0..source.len())));
    assert_eq!(lexer.span, Span::new(0..source.len()));

    // We covered the whole source
    lexer.next();
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}
