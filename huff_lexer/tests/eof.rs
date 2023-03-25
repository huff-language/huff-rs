use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn end_of_file() {
    let source = " ";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source.source.clone());

    // Eats the whitespace
    let _ = lexer.next();

    // Get an EOF token
    let tok = lexer.next();
    let tok = tok.unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Eof, Span::new(0..0, None)));

    // We should have reached EOF now
    assert!(lexer.eof);
    assert!(lexer.next().is_none());
}
