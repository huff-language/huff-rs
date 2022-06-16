use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn jump_destinations() {
    let source = "here: RUN()";
    let mut lexer = Lexer::new(source);

    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    assert_eq!(unwrapped, Token::new(TokenKind::JumpDest("here:"), Span::new(0..5)));
}

#[test]
fn jump_labels() {
    let source = "<label> jumpi";
    let mut lexer = Lexer::new(source);

    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    assert_eq!(unwrapped, Token::new(TokenKind::JumpLabel("<label>"), Span::new(0..7)));
}
