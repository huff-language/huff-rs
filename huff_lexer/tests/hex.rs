use bytes::{BytesMut};
use huff_lexer::*;
use huff_utils::prelude::*;

fn str_to_array(s: &str) -> [u8; 32] {
    let mut arr: [u8; 32] = Default::default();
    let mut buf = BytesMut::from(s);
    buf.resize(32, 0);
    arr.copy_from_slice(buf.as_ref());
    arr
}

#[test]
fn parses_single_hex() {
    let source = "0x1234";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // The first and only token should be lexed as Hex(0x1234)
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Literal(str_to_array("0x1234")), Span::new(0..6)));
    assert_eq!(lexer.span, Span::new(0..6));

    // We covered the whole source
    lexer.next();
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}
