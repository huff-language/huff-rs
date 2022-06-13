use huff_lexer::Lexer;
use huff_utils::prelude::{Span, Token, TokenKind};

#[test]
fn parses_builtin_function_in_macro_body() {
    let builtin_funcs = ["__codesize", "__tablesize", "__tablestart"];

    for builtin in builtin_funcs {
        let source = format!(
            r#"
            #define macro TEST() = takes(0) returns(0) {}
                {}(MAIN)
            {}
            "#,
            "{", builtin, "}",
        );
        let mut lexer = Lexer::new(source.as_str());
        assert_eq!(lexer.source, source);

        let _ = lexer.next(); // whitespace
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
        let _ = lexer.next(); // takes
        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // 0
        let _ = lexer.next(); // close parenthesis
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // returns
        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // 0
        let _ = lexer.next(); // close parenthesis
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // {
        let _ = lexer.next(); // whitespace

        // The builtin fn should be parsed as a `TokenKind::BuiltinFunction` here.
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let builtin_span = Span::new(74..74 + builtin.len());
        assert_eq!(
            unwrapped,
            Token::new(TokenKind::BuiltinFunction(builtin.to_string()), builtin_span)
        );
        assert_eq!(lexer.span, builtin_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // MAIN
        let _ = lexer.next(); // close parenthesis
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // }
        let _ = lexer.next(); // whitespace

        // We covered the whole source
        assert_eq!(lexer.span.end, source.len());
        assert!(lexer.eof);
    }
}

#[test]
#[should_panic]
fn fails_to_parse_builtin_outside_macro_body() {
    let builtin_funcs = ["__codesize", "__tablesize", "__tablestart"];

    for builtin in builtin_funcs {
        let source = format!("{}(MAIN)", builtin);
        let mut lexer = Lexer::new(source.as_str());
        assert_eq!(lexer.source, source);

        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let fn_name_span = Span::new(0..builtin.len());
        assert_eq!(
            unwrapped,
            Token::new(TokenKind::BuiltinFunction(builtin.to_string()), fn_name_span)
        );
        assert_eq!(lexer.span, fn_name_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // MAIN
        let _ = lexer.next(); // close parenthesis

        // We covered the whole source
        assert_eq!(lexer.span.end, source.len());
        assert!(lexer.eof);
    }
}
