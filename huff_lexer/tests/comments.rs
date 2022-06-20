use huff_lexer::*;
use huff_utils::prelude::*;

// use proptest::prelude::*;

// proptest! {
//     #[test]
//     fn doesnt_crash(s in "\\PC*") {
//         parse_date(&s);
//     }
// }

#[test]
fn instantiates() {
    let source = "#define macro HELLO_WORLD()";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.clone());
    assert_eq!(lexer.source, flattened_source);
    assert_eq!(lexer.span, Span::default());
    assert!(!lexer.eof);
}

#[test]
fn single_line_comments() {
    let source = "// comment contents \n#define macro HELLO_WORLD()";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source.clone());
    assert_eq!(lexer.source, flattened_source);

    // The first token should be a single line comment
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    assert_eq!(
        unwrapped,
        Token::new(TokenKind::Comment("// comment contents ".to_string()), Span::new(0..20, None))
    );
    assert_eq!(lexer.span, Span::new(0..20, None));

    // The second token should be the newline character parsed as a whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(20..21, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, define_span.clone()));
    assert_eq!(lexer.span, define_span);

    // This token should be a Define identifier
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(21..28, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span.clone()));
    assert_eq!(lexer.span, define_span);

    // The next token should be the whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(28..29, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, define_span.clone()));
    assert_eq!(lexer.span, define_span);

    // Then we should parse the macro keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let macro_span = Span::new(29..34, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Macro, macro_span.clone()));
    assert_eq!(lexer.span, macro_span);

    // The next token should be another whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let ws_span = Span::new(34..35, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, ws_span.clone()));
    assert_eq!(lexer.span, ws_span);

    // Then we should get the function name
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let function_span = Span::new(35..46, None);
    assert_eq!(
        unwrapped,
        Token::new(TokenKind::Ident("HELLO_WORLD".to_string()), function_span.clone())
    );
    assert_eq!(lexer.span, function_span);

    // Then we should have an open paren
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let open_paren_span = Span::new(46..47, None);
    assert_eq!(unwrapped, Token::new(TokenKind::OpenParen, open_paren_span.clone()));
    assert_eq!(lexer.span, open_paren_span);

    // Lastly, we should have a closing parenthesis
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let close_paren_span = Span::new(47..48, None);
    assert_eq!(unwrapped, Token::new(TokenKind::CloseParen, close_paren_span.clone()));
    assert_eq!(lexer.span, close_paren_span);

    // We covered the whole source
    assert!(lexer.eof);
    assert_eq!(source.len(), 48);
}

#[test]
fn multi_line_comments() {
    let source = "/* comment contents*/#define macro HELLO_WORLD()";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source.clone());
    assert_eq!(lexer.source, flattened_source);

    // The first token should be a single line comment
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    assert_eq!(
        unwrapped,
        Token::new(TokenKind::Comment("/* comment contents*/".to_string()), Span::new(0..21, None))
    );
    assert_eq!(lexer.span, Span::new(0..21, None));

    // This token should be a Define identifier
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(21..28, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span.clone()));
    assert_eq!(lexer.span, define_span);

    // The next token should be the whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(28..29, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, define_span.clone()));
    assert_eq!(lexer.span, define_span);

    // Then we should parse the macro keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let macro_span = Span::new(29..34, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Macro, macro_span.clone()));
    assert_eq!(lexer.span, macro_span);

    // The next token should be another whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let ws_span = Span::new(34..35, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, ws_span.clone()));
    assert_eq!(lexer.span, ws_span);

    // Then we should get the function name
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let function_span = Span::new(35..46, None);
    assert_eq!(
        unwrapped,
        Token::new(TokenKind::Ident("HELLO_WORLD".to_string()), function_span.clone())
    );
    assert_eq!(lexer.span, function_span);

    // Then we should have an open paren
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let open_paren_span = Span::new(46..47, None);
    assert_eq!(unwrapped, Token::new(TokenKind::OpenParen, open_paren_span.clone()));
    assert_eq!(lexer.span, open_paren_span);

    // Lastly, we should have a closing parenthesis
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let close_paren_span = Span::new(47..48, None);
    assert_eq!(unwrapped, Token::new(TokenKind::CloseParen, close_paren_span.clone()));
    assert_eq!(lexer.span, close_paren_span);

    // We covered the whole source
    assert!(lexer.eof);
    assert_eq!(source.len(), 48);
}
