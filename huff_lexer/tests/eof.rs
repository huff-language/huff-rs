use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn end_of_file() {
    let source = " ";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Eats the whitespace
    let _ = lexer.next();

    // Get an EOF token
    let tok = lexer.next();
    let tok = tok.unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Eof, Span::new(1..1)));
    assert_eq!(lexer.span, Span::new(1..1));

    // We should have reached EOF now
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
    assert!(lexer.next().is_none());
}
