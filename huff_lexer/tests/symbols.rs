use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn lexes_assign_op() {
    let source = "#define constant TRANSFER_EVENT_SIGNATURE =";
    // comment contents \n#define macro HELLO_WORLD()";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // This token should be a Define identifier
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(0..7);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span));
    assert_eq!(lexer.span, define_span);

    // The next token should be the whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(7..8);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, define_span));
    assert_eq!(lexer.span, define_span);

    // Then we should parse the constant keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let constant_span = Span::new(8..16);
    assert_eq!(unwrapped, Token::new(TokenKind::Constant, constant_span));
    assert_eq!(lexer.span, constant_span);

    // The next token should be another whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let ws_span = Span::new(16..17);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, ws_span));
    assert_eq!(lexer.span, ws_span);

    // Then we should get the function name
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let function_span = Span::new(17..41);
    assert_eq!(unwrapped, Token::new(TokenKind::Ident("TRANSFER_EVENT_SIGNATURE"), function_span));
    assert_eq!(lexer.span, function_span);

    // Then we should have another whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let whitespace_span = Span::new(41..42);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, whitespace_span));
    assert_eq!(lexer.span, whitespace_span);

    // Finally, we have our assign operator
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let assign_span = Span::new(42..43);
    assert_eq!(unwrapped, Token::new(TokenKind::Assign, assign_span));
    assert_eq!(lexer.span, assign_span);

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
    assert!(lexer.next().is_none());
}
