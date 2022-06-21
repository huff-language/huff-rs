use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn parses_macro_keyword() {
    let source = "#define macro";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let define_span = Span::new(0..7);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span));
    assert_eq!(lexer.span, define_span);

    // Lastly we should parse the macro keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let macro_span = Span::new(8..13);
    assert_eq!(unwrapped, Token::new(TokenKind::Macro, macro_span));
    assert_eq!(lexer.span, macro_span);

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    lexer.next();
    assert!(lexer.eof);
}

#[test]
fn parses_function_keyword() {
    let source = "#define function";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let define_span = Span::new(0..7);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span));
    assert_eq!(lexer.span, define_span);

    // Lastly we should parse the function keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let function_span = Span::new(8..16);
    assert_eq!(unwrapped, Token::new(TokenKind::Function, function_span));
    assert_eq!(lexer.span, function_span);

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    lexer.next();
    assert!(lexer.eof);
}

#[test]
fn parses_event_keyword() {
    let source = "#define event TestEvent(uint256)";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let define_span = Span::new(0..7);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span));
    assert_eq!(lexer.span, define_span);

    // Lastly we should parse the event keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let event_span = Span::new(8..13);
    assert_eq!(unwrapped, Token::new(TokenKind::Event, event_span));
    assert_eq!(lexer.span, event_span);

    let _ = lexer.next(); // event name
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // uint256
    let _ = lexer.next(); // close parenthesis

    // We covered the whole source
    assert_eq!(lexer.next(), None);
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_constant_keyword() {
    let source = "#define constant";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let define_span = Span::new(0..7);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span));
    assert_eq!(lexer.span, define_span);

    // Lastly we should parse the constant keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let constant_span = Span::new(8..16);
    assert_eq!(unwrapped, Token::new(TokenKind::Constant, constant_span));
    assert_eq!(lexer.span, constant_span);

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    lexer.next();
    assert!(lexer.eof);
}

#[test]
fn parses_takes_and_returns_keywords() {
    let source = "#define macro TEST() = takes (0) returns (0)";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    let _ = lexer.next(); // #define
    let _ = lexer.next(); // macro
    let _ = lexer.next(); // TEST
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // close parenthesis
    let _ = lexer.next(); // =

    // Lex Takes First
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let takes_span = Span::new(23..28);
    assert_eq!(unwrapped, Token::new(TokenKind::Takes, takes_span));
    assert_eq!(lexer.span, takes_span);

    // Lex the middle 5 chars
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis

    // Lex Returns
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let returns_span = Span::new(33..40);
    assert_eq!(unwrapped, Token::new(TokenKind::Returns, returns_span));
    assert_eq!(lexer.span, returns_span);

    // Lex the last 4 chars
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    lexer.next();
    assert!(lexer.eof);
}

#[test]
fn parses_takes_and_returns_keywords_tight_syntax() {
    let source = "#define macro TEST() = takes(0) returns(0)";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    let _ = lexer.next(); // #define
    let _ = lexer.next(); // macro
    let _ = lexer.next(); // TEST
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // close parenthesis
    let _ = lexer.next(); // =

    // Lex Takes First
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let takes_span = Span::new(23..28);
    assert_eq!(unwrapped, Token::new(TokenKind::Takes, takes_span));
    assert_eq!(lexer.span, takes_span);

    // Lex the next 4 chars
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis

    // Lex Returns
    let tok = lexer.next();
    let unwrapped = tok.unwrap();
    let returns_span = Span::new(32..39);
    assert_eq!(unwrapped, Token::new(TokenKind::Returns, returns_span));
    assert_eq!(lexer.span, returns_span);

    // Lex the last 3 chars
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    lexer.next();
    assert!(lexer.eof);
}

#[test]
fn parses_function_type_keywords() {
    let source = "#define function test() view returns (uint256)";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    let _ = lexer.next(); // #define
    let _ = lexer.next(); // function
    let _ = lexer.next(); // test
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // close parenthesis

    // Lex view first
    let tok = lexer.next().unwrap();
    let view_span = Span::new(24..28);
    assert_eq!(tok, Token::new(TokenKind::View, view_span));
    assert_eq!(lexer.span, view_span);

    // Lex the next tokens
    let _ = lexer.next(); // returns
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // uint256
    let _ = lexer.next(); // paren

    // We covered the whole source
    assert_eq!(lexer.next(), None);
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}
