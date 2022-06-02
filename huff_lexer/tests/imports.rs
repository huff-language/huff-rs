use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn include_no_quotes() {
    let source = "#include";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // The first token should be a single line comment
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    assert_eq!(unwrapped, Token::new(TokenKind::Include, Span::new(0..8)));
    assert_eq!(lexer.span, Span::new(0..8));
    assert!(lexer.eof);
    assert!(lexer.next().is_none());
}

#[test]
fn include_with_string() {
    let source = "#include \"./huffs/Ownable.huff\"";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // The first token should be a single line comment
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    assert_eq!(unwrapped, Token::new(TokenKind::Include, Span::new(0..8)));
    assert_eq!(lexer.span, Span::new(0..8));

    // Lex the whitespace char
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let literal_span = Span::new(8..9);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, literal_span));
    assert_eq!(lexer.span, literal_span);

    // Then we should parse the string literal
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let literal_span = Span::new(9..31);
    assert_eq!(unwrapped, Token::new(TokenKind::Str("./huffs/Ownable.huff"), literal_span));
    assert_eq!(lexer.span, literal_span);

    // We should have reached EOF now
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
    assert!(lexer.next().is_none());
}

#[test]
fn include_with_string_single_quote() {
    let source = "#include './huffs/Ownable.huff'";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // The first token should be a single line comment
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    assert_eq!(unwrapped, Token::new(TokenKind::Include, Span::new(0..8)));
    assert_eq!(lexer.span, Span::new(0..8));

    // Lex the whitespace char
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let literal_span = Span::new(8..9);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, literal_span));
    assert_eq!(lexer.span, literal_span);

    // Then we should parse the string literal
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let literal_span = Span::new(9..31);
    assert_eq!(unwrapped, Token::new(TokenKind::Str("./huffs/Ownable.huff"), literal_span));
    assert_eq!(lexer.span, literal_span);

    // We should have reached EOF now
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
    assert!(lexer.next().is_none());
}
