use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn parses_macro_keyword() {
    let source = "#define macro";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(0..7);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span));
    assert_eq!(lexer.span, define_span);

    // The next token should be the whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let whitespace_span = Span::new(7..8);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, whitespace_span));
    assert_eq!(lexer.span, whitespace_span);

    // Lastly we should parse the macro keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let macro_span = Span::new(8..13);
    assert_eq!(unwrapped, Token::new(TokenKind::Macro, macro_span));
    assert_eq!(lexer.span, macro_span);

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_function_keyword() {
    let source = "#define function";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(0..7);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span));
    assert_eq!(lexer.span, define_span);

    // The next token should be the whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let whitespace_span = Span::new(7..8);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, whitespace_span));
    assert_eq!(lexer.span, whitespace_span);

    // Lastly we should parse the function keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let function_span = Span::new(8..16);
    assert_eq!(unwrapped, Token::new(TokenKind::Function, function_span));
    assert_eq!(lexer.span, function_span);

    // We covered the whole source
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
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(0..7);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span));
    assert_eq!(lexer.span, define_span);

    // The next token should be the whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let whitespace_span = Span::new(7..8);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, whitespace_span));
    assert_eq!(lexer.span, whitespace_span);

    // Lastly we should parse the constant keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let constant_span = Span::new(8..16);
    assert_eq!(unwrapped, Token::new(TokenKind::Constant, constant_span));
    assert_eq!(lexer.span, constant_span);

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_takes_and_returns_keywords() {
    let source = "takes (0) returns (0)";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Lex Takes First
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let takes_span = Span::new(0..5);
    assert_eq!(unwrapped, Token::new(TokenKind::Takes, takes_span));
    assert_eq!(lexer.span, takes_span);

    // Lex the middle 5 chars
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis
    let _ = lexer.next(); // whitespace

    // Lex Returns
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let returns_span = Span::new(10..17);
    assert_eq!(unwrapped, Token::new(TokenKind::Returns, returns_span));
    assert_eq!(lexer.span, returns_span);

    // Lex the last 4 chars
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_takes_and_returns_keywords_tight_syntax() {
    let source = "takes(0) returns(0)";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Lex Takes First
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let takes_span = Span::new(0..5);
    assert_eq!(unwrapped, Token::new(TokenKind::Takes, takes_span));
    assert_eq!(lexer.span, takes_span);

    // Lex the next 4 chars
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis
    let _ = lexer.next(); // whitespace

    // Lex Returns
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let returns_span = Span::new(9..16);
    assert_eq!(unwrapped, Token::new(TokenKind::Returns, returns_span));
    assert_eq!(lexer.span, returns_span);

    // Lex the last 3 chars
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_function_definition_with_keyword_name() {
    let key_words = ["macro", "function", "constant", "takes", "returns"];

    for s in key_words {
        let source = format!("#define function {}(uint256) takes(0) returns(0)", s);
        let mut lexer = Lexer::new(source.as_str());
        assert_eq!(lexer.source, source);

        let end_span_s = 17 + s.len();

        let _ = lexer.next(); // #define
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // function
        let _ = lexer.next(); // whitespace

        // Keyword as a function name (s)
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let ident_span = Span::new(17..end_span_s);
        assert_eq!(unwrapped, Token::new(TokenKind::Ident(&s), ident_span));
        assert_eq!(lexer.span, ident_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // uint256
        let _ = lexer.next(); // close parenthesis
        let _ = lexer.next(); // whitespace

        // Ensure that this "takes" is lexed as a `TokenKind::Takes`
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let takes_span = Span::new((end_span_s + 10)..(end_span_s + 15));
        assert_eq!(unwrapped, Token::new(TokenKind::Takes, takes_span));
        assert_eq!(lexer.span, takes_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // 0 (TokenKind::Num)
        let _ = lexer.next(); // close parenthesis
        let _ = lexer.next(); // whitespace

        // Ensure that this "returns" is lexed as a `TokenKind::Returns`
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let returns_span = Span::new((end_span_s + 19)..(end_span_s + 26));
        assert_eq!(unwrapped, Token::new(TokenKind::Returns, returns_span));
        assert_eq!(lexer.span, returns_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // 0 (TokenKind::Num)
        let _ = lexer.next(); // close parenthesis

        // We covered the whole source
        assert_eq!(lexer.span.end, source.len());
        assert!(lexer.eof);
        assert!(lexer.next().is_none());
    }
}

#[test]
fn parses_function_with_keyword_name_in_main_macro() {
    let key_words = ["macro", "function", "constant", "takes", "returns"];

    for s in key_words {
        // ex:
        // takes:
        //     TAKES()
        let source = format!(
            r#"{}:
            {}()"#,
            s,
            s.to_uppercase()
        );
        let mut lexer = Lexer::new(source.as_str());
        assert_eq!(lexer.source, source);

        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let fn_name_span = Span::new(0..s.len());
        assert_eq!(unwrapped, Token::new(TokenKind::Ident(&s), fn_name_span));
        assert_eq!(lexer.span, fn_name_span);

        let _ = lexer.next(); // :
        let _ = lexer.next(); // whitespace

        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let fn_name_span = Span::new((s.len() + 14)..(s.len() * 2 + 14));
        assert_eq!(unwrapped, Token::new(TokenKind::Ident(&s.to_uppercase()), fn_name_span));
        assert_eq!(lexer.span, fn_name_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // close parenthesis

        // We covered the whole source
        assert_eq!(lexer.span.end, source.len());
        assert!(lexer.eof);
        assert!(lexer.next().is_none());
    }
}
