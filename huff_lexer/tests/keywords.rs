use huff_lexer::*;
use huff_utils::prelude::*;
use std::ops::Deref;

#[test]
fn parses_macro_keyword() {
    let source = "#define macro";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(0..7, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span.clone()));
    assert_eq!(lexer.current_span().deref(), &define_span);

    // The next token should be the whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let whitespace_span = Span::new(7..8, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, whitespace_span.clone()));
    assert_eq!(lexer.current_span().deref(), &whitespace_span);

    // Lastly we should parse the macro keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let macro_span = Span::new(8..13, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Macro, macro_span.clone()));
    assert_eq!(lexer.current_span().deref(), &macro_span);

    // We covered the whole source
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_fn_keyword() {
    let source = "#define fn";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(0..7, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span.clone()));
    assert_eq!(lexer.current_span().deref(), &define_span);

    // The next token should be the whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let whitespace_span = Span::new(7..8, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, whitespace_span.clone()));
    assert_eq!(lexer.current_span().deref(), &whitespace_span);

    // Lastly we should parse the fn keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let fn_span = Span::new(8..10, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Fn, fn_span.clone()));
    assert_eq!(lexer.current_span().deref(), &fn_span);

    // We covered the whole source
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_test_keyword() {
    let source = "#define test";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(0..7, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span.clone()));
    assert_eq!(lexer.current_span().deref(), &define_span);

    // The next token should be the whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let whitespace_span = Span::new(7..8, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, whitespace_span.clone()));
    assert_eq!(lexer.current_span().deref(), &whitespace_span);

    // Lastly we should parse the fn keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let test_span = Span::new(8..12, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Test, test_span.clone()));
    assert_eq!(lexer.current_span().deref(), &test_span);

    // We covered the whole source
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_function_keyword() {
    let source = "#define function";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(0..7, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span.clone()));
    assert_eq!(lexer.current_span().deref(), &define_span);

    // The next token should be the whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let whitespace_span = Span::new(7..8, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, whitespace_span.clone()));
    assert_eq!(lexer.current_span().deref(), &whitespace_span);

    // Lastly we should parse the function keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let function_span = Span::new(8..16, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Function, function_span.clone()));
    assert_eq!(lexer.current_span().deref(), &function_span);

    // We covered the whole source
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_event_keyword() {
    let source = "#define event TestEvent(uint256)";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(0..7, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span.clone()));
    assert_eq!(lexer.current_span().deref(), &define_span);

    // The next token should be the whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let whitespace_span = Span::new(7..8, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, whitespace_span.clone()));
    assert_eq!(lexer.current_span().deref(), &whitespace_span);

    // Lastly we should parse the event keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let event_span = Span::new(8..13, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Event, event_span.clone()));
    assert_eq!(lexer.current_span().deref(), &event_span);

    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // event name
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // uint256
    let _ = lexer.next(); // close parenthesis

    // We covered the whole source
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_constant_keyword() {
    let source = "#define constant";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(0..7, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span.clone()));
    assert_eq!(lexer.current_span().deref(), &define_span);

    // The next token should be the whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let whitespace_span = Span::new(7..8, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, whitespace_span.clone()));
    assert_eq!(lexer.current_span().deref(), &whitespace_span);

    // Lastly we should parse the constant keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let constant_span = Span::new(8..16, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Constant, constant_span.clone()));
    assert_eq!(lexer.current_span().deref(), &constant_span);

    // We covered the whole source
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_takes_and_returns_keywords() {
    let source = "#define macro TEST() = takes   (0)   returns   (0)";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    let _ = lexer.next(); // #define
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // macro
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // TEST
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // close parenthesis
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // =
    let _ = lexer.next(); // whitespace

    // Lex Takes First
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let takes_span = Span::new(23..28, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Takes, takes_span.clone()));
    assert_eq!(lexer.current_span().deref(), &takes_span);

    // Lex the middle 5 chars
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis
    let _ = lexer.next(); // whitespace

    // Lex Returns
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let returns_span = Span::new(37..44, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Returns, returns_span.clone()));
    assert_eq!(lexer.current_span().deref(), &returns_span);

    // Lex the last 4 chars
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis

    // We covered the whole source
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_takes_and_returns_keywords_tight_syntax() {
    let source = "#define macro TEST() = takes(0) returns(0)";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    let _ = lexer.next(); // #define
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // macro
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // TEST
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // close parenthesis
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // =
    let _ = lexer.next(); // whitespace

    // Lex Takes First
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let takes_span = Span::new(23..28, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Takes, takes_span.clone()));
    assert_eq!(lexer.current_span().deref(), &takes_span);

    // Lex the next 4 chars
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis
    let _ = lexer.next(); // whitespace

    // Lex Returns
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let returns_span = Span::new(32..39, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Returns, returns_span.clone()));
    assert_eq!(lexer.current_span().deref(), &returns_span);

    // Lex the last 3 chars
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis

    // We covered the whole source
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_function_type_keywords() {
    let source = "#define function test() view returns (uint256)";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    let _ = lexer.next(); // #define
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // function
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // test
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // close parenthesis
    let _ = lexer.next(); // whitespace

    // Lex view first
    let tok = lexer.next().unwrap().unwrap();
    let view_span = Span::new(24..28, None);
    assert_eq!(tok, Token::new(TokenKind::View, view_span.clone()));
    assert_eq!(lexer.current_span().deref(), &view_span);

    // Lex the next 4 chars
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // returns
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // uint256
    let _ = lexer.next(); // paren

    // We covered the whole source
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_function_definition_with_keyword_name() {
    let key_words = [
        "macro",
        "fn",
        "test",
        "function",
        "constant",
        "error",
        "takes",
        "returns",
        "define",
        "include",
        "nonpayable",
        "payable",
        "view",
        "pure",
        "jumptable",
        "jumptable__packed",
        "table",
    ];

    for s in key_words {
        let source = &format!("#define function {}(uint256) view returns(uint256)", s);
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let mut lexer = Lexer::new(flattened_source);

        let end_span_s = 17 + s.len();

        let _ = lexer.next(); // #define
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // function
        let _ = lexer.next(); // whitespace

        // Keyword as a function name (s)
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let ident_span = Span::new(17..end_span_s, None);
        assert_eq!(unwrapped, Token::new(TokenKind::Ident(s.to_string()), ident_span.clone()));
        assert_eq!(lexer.current_span().deref(), &ident_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // uint256
        let _ = lexer.next(); // close parenthesis
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // view
        let _ = lexer.next(); // whitespace

        // Ensure that this "returns" is lexed as a `TokenKind::Returns`
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let returns_span = Span::new((end_span_s + 15)..(end_span_s + 22), None);
        assert_eq!(unwrapped, Token::new(TokenKind::Returns, returns_span.clone()));
        assert_eq!(lexer.current_span().deref(), &returns_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // uint256
        let _ = lexer.next(); // close parenthesis

        // We covered the whole source
        assert_eq!(lexer.current_span().end, source.len());
        assert!(lexer.eof);
    }
}

#[test]
fn parses_label_with_keyword_name() {
    let key_words = [
        "macro",
        "fn",
        "test",
        "function",
        "constant",
        "error",
        "takes",
        "returns",
        "define",
        "include",
        "nonpayable",
        "payable",
        "view",
        "pure",
        "jumptable",
        "jumptable__packed",
        "table",
    ];

    for s in key_words {
        // ex:
        // takes:
        //     TAKES()
        let source = &format!(
            r#"{}:
            {}()"#,
            s,
            s.to_uppercase()
        );

        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let mut lexer = Lexer::new(flattened_source);

        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let fn_name_span = Span::new(0..s.len(), None);
        assert_eq!(unwrapped, Token::new(TokenKind::Label(s.to_string()), fn_name_span.clone()));
        assert_eq!(lexer.current_span().deref(), &fn_name_span);

        let _ = lexer.next(); // colon
        let _ = lexer.next(); // whitespace

        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let fn_name_span = Span::new((s.len() + 14)..(s.len() * 2 + 14), None);
        assert_eq!(unwrapped, Token::new(TokenKind::Ident(s.to_uppercase()), fn_name_span.clone()));
        assert_eq!(lexer.current_span().deref(), &fn_name_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // close parenthesis

        // We covered the whole source
        assert_eq!(lexer.current_span().end, source.len());
        assert!(lexer.eof);
    }
}

#[test]
fn parses_function_with_keyword_name() {
    let key_words = [
        "macro",
        "fn",
        "test",
        "function",
        "constant",
        "error",
        "takes",
        "returns",
        "define",
        "include",
        "nonpayable",
        "payable",
        "view",
        "pure",
        "jumptable",
        "jumptable__packed",
        "table",
    ];

    for s in key_words {
        let source = &format!("dup1 0x7c09063f eq {} jumpi", s);

        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let mut lexer = Lexer::new(flattened_source);

        let _ = lexer.next(); // dup1
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // function sig (0x7c09063f is for `takes`, but doesn't matter here)
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // eq
        let _ = lexer.next(); // whitespace

        // The keyword should be parsed as a `TokenKind::Ident` here.
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let fn_name_span = Span::new(19..19 + s.len(), None);
        assert_eq!(unwrapped, Token::new(TokenKind::Ident(s.to_string()), fn_name_span.clone()));
        assert_eq!(lexer.current_span().deref(), &fn_name_span);

        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // jumpi

        // We covered the whole source
        assert_eq!(lexer.current_span().end, source.len());
        assert!(lexer.eof);
    }
}

#[test]
fn parses_function_with_keyword_name_in_macro() {
    let key_words = [
        "macro",
        "fn",
        "test",
        "function",
        "constant",
        "error",
        "takes",
        "returns",
        "define",
        "include",
        "nonpayable",
        "payable",
        "view",
        "pure",
        "jumptable",
        "jumptable__packed",
        "table",
    ];

    for s in key_words {
        let source = &format!(
            r#"
            #define macro NUMS() = takes(0) returns(1) {}
                0x01 0x02 {}
            {}
            "#,
            "{", s, "}",
        );

        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let mut lexer = Lexer::new(flattened_source);

        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // #define
        let _ = lexer.next(); // whitespace

        // Ensure "macro" is parsed as a keyword here
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let takes_span = Span::new(21..26, None);
        assert_eq!(unwrapped, Token::new(TokenKind::Macro, takes_span.clone()));
        assert_eq!(lexer.current_span().deref(), &takes_span);

        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // NUMS
        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // close parenthesis
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // =
        let _ = lexer.next(); // whitespace

        // Ensure "takes" is parsed as a keyword here
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let takes_span = Span::new(36..41, None);
        assert_eq!(unwrapped, Token::new(TokenKind::Takes, takes_span.clone()));
        assert_eq!(lexer.current_span().deref(), &takes_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // 0
        let _ = lexer.next(); // close parenthesis
        let _ = lexer.next(); // whitespace

        // Ensure "returns" is parsed as a keyword here
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let returns_span = Span::new(45..52, None);
        assert_eq!(unwrapped, Token::new(TokenKind::Returns, returns_span.clone()));
        assert_eq!(lexer.current_span().deref(), &returns_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // 1
        let _ = lexer.next(); // close parenthesis
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // {
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // 0x01
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // 0x02
        let _ = lexer.next(); // whitespace

        // The keyword should be parsed as a `TokenKind::Ident` here.
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let fn_name_span = Span::new(84..84 + s.len(), None);
        assert_eq!(unwrapped, Token::new(TokenKind::Ident(s.to_string()), fn_name_span.clone()));
        assert_eq!(lexer.current_span().deref(), &fn_name_span);

        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // }
        let _ = lexer.next(); // whitespace

        // We covered the whole source
        assert_eq!(lexer.current_span().end, source.len());
        assert!(lexer.eof);
    }
}

#[test]
fn parses_keyword_arbitrary_whitespace() {
    // Macro, constant, and function keywords first- they are all preceded by "#define"
    let key_words = [
        ("macro", TokenKind::Macro),
        ("fn", TokenKind::Fn),
        ("test", TokenKind::Test),
        ("constant", TokenKind::Constant),
        ("error", TokenKind::Error),
        ("function", TokenKind::Function),
    ];

    for (key, kind) in key_words {
        let source = &format!("#define     {}", key);

        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let mut lexer = Lexer::new(flattened_source);

        // Define Identifier first
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let define_span = Span::new(0..7, None);
        assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span.clone()));
        assert_eq!(lexer.current_span().deref(), &define_span);

        // The next token should be the whitespace
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let whitespace_span = Span::new(7..12, None);
        assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, whitespace_span.clone()));
        assert_eq!(lexer.current_span().deref(), &whitespace_span);

        // Lastly we should parse the constant keyword
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let constant_span = Span::new(12..12 + key.len(), None);
        assert_eq!(unwrapped, Token::new(kind, constant_span.clone()));
        assert_eq!(lexer.current_span().deref(), &constant_span);

        // We covered the whole source
        assert_eq!(lexer.current_span().end, source.len());
        assert!(lexer.eof);
    }
}

#[test]
fn parses_takes_keyword_arbitrary_whitespace() {
    let source = "#define macro TEST() =      takes (0) returns (0)";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    let _ = lexer.next(); // #define
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // macro
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // TEST
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // close parenthesis
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // =
    let _ = lexer.next(); // whitespace

    // Lex Takes First
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let takes_span = Span::new(28..33, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Takes, takes_span.clone()));
    assert_eq!(lexer.current_span().deref(), &takes_span);

    // Lex the middle 5 chars
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis
    let _ = lexer.next(); // whitespace

    // Lex Returns
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let returns_span = Span::new(38..45, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Returns, returns_span.clone()));
    assert_eq!(lexer.current_span().deref(), &returns_span);

    // Lex the last 4 chars
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // open parenthesis
    let _ = lexer.next(); // 0
    let _ = lexer.next(); // close parenthesis

    // We covered the whole source
    assert_eq!(lexer.current_span().end, source.len());
    assert!(lexer.eof);
}

#[test]
fn parses_define_with_extra_suffix() {
    let source = "#defineabc";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let span = Span::new(0..7, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, span.clone()));
    assert_eq!(lexer.current_span().deref(), &span);
}

#[test]
fn parses_include_with_extra_suffix() {
    let source = "#includeabc";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source);

    // Define Identifier first
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let span = Span::new(0..8, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Include, span.clone()));
    assert_eq!(lexer.current_span().deref(), &span);
}
