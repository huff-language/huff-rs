use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn instantiates() {
    let source = "#define macro HELLO_WORLD()";
    let lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);
    assert_eq!(lexer.span, Span::default());
    assert!(!lexer.eof);
}

#[test]
fn single_line_comments() {
    let source = "// comment contents \n#define macro HELLO_WORLD()";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // The first token should be a single line comment
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    assert_eq!(unwrapped, Token::new(TokenKind::Comment("// comment contents "), Span::new(0..20)));
    assert_eq!(lexer.span, Span::new(0..20));

    // This token should be a Define identifier
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let define_span = Span::new(21..28);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span));
    assert_eq!(lexer.span, define_span);

    // Then we should parse the macro keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let macro_span = Span::new(29..34);
    assert_eq!(unwrapped, Token::new(TokenKind::Macro, macro_span));
    assert_eq!(lexer.span, macro_span);

    // Then we should get the function name
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let function_span = Span::new(35..46);
    assert_eq!(unwrapped, Token::new(TokenKind::Ident("HELLO_WORLD"), function_span));
    assert_eq!(lexer.span, function_span);

    // Then we should have an open paren
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let open_paren_span = Span::new(46..47);
    assert_eq!(unwrapped, Token::new(TokenKind::OpenParen, open_paren_span));
    assert_eq!(lexer.span, open_paren_span);

    // Lastly, we should have a closing parenthesis
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let close_paren_span = Span::new(47..48);
    assert_eq!(unwrapped, Token::new(TokenKind::CloseParen, close_paren_span));
    assert_eq!(lexer.span, close_paren_span);

    // We covered the whole source
    let tok = lexer.next();
    assert!(tok.is_none());
    assert!(lexer.eof);
    assert_eq!(source.len(), 48);
}

#[test]
fn multi_line_comments() {
    let source = "/* comment contents*/#define macro HELLO_WORLD()";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // The first token should be a single line comment
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    assert_eq!(
        unwrapped,
        Token::new(TokenKind::Comment("/* comment contents*/"), Span::new(0..21))
    );
    assert_eq!(lexer.span, Span::new(0..21));

    // This token should be a Define identifier
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let define_span = Span::new(21..28);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span));
    assert_eq!(lexer.span, define_span);

    // Then we should parse the macro keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let macro_span = Span::new(29..34);
    assert_eq!(unwrapped, Token::new(TokenKind::Macro, macro_span));
    assert_eq!(lexer.span, macro_span);

    // Then we should get the function name
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let function_span = Span::new(35..46);
    assert_eq!(unwrapped, Token::new(TokenKind::Ident("HELLO_WORLD"), function_span));
    assert_eq!(lexer.span, function_span);

    // Then we should have an open paren
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let open_paren_span = Span::new(46..47);
    assert_eq!(unwrapped, Token::new(TokenKind::OpenParen, open_paren_span));
    assert_eq!(lexer.span, open_paren_span);

    // Lastly, we should have a closing parenthesis
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let close_paren_span = Span::new(47..48);
    assert_eq!(unwrapped, Token::new(TokenKind::CloseParen, close_paren_span));
    assert_eq!(lexer.span, close_paren_span);

    // We covered the whole source
    assert!(lexer.next().is_none());
    assert!(lexer.eof);
    assert_eq!(source.len(), 48);
}
