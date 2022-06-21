use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn label() {
    let source = "here: RUN()";
    let mut lexer = Lexer::new(source);

    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    assert_eq!(unwrapped, Token::new(TokenKind::Label("here:"), Span::new(0..5)));
}
