use huff_lexer::Lexer;
use huff_utils::prelude::{FullFileSource, Span, Token, TokenKind};
use std::ops::Deref;

#[test]
fn parses_builtin_function_in_macro_body() {
    let builtin_funcs = [
        "__codesize",
        "__tablesize",
        "__tablestart",
        "__FUNC_SIG",
        "__EVENT_HASH",
        "__ERROR",
        "__RIGHTPAD",
    ];

    for builtin in builtin_funcs {
        let source = &format!(
            r#"
            #define macro TEST() = takes(0) returns(0) {}
                {}(MAIN)
            {}
            "#,
            "{", builtin, "}",
        );
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let mut lexer = Lexer::new(flattened_source.clone());
        assert_eq!(lexer.source, flattened_source);

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
        let builtin_span = Span::new(74..74 + builtin.len(), None);
        assert_eq!(
            unwrapped,
            Token::new(TokenKind::BuiltinFunction(builtin.to_string()), builtin_span.clone())
        );
        assert_eq!(lexer.current_span().deref(), &builtin_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // MAIN
        let _ = lexer.next(); // close parenthesis
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // }
        let _ = lexer.next(); // whitespace

        // We covered the whole source
        assert_eq!(lexer.current_span().end, source.len());
        assert!(lexer.eof);
    }
}

#[test]
#[should_panic]
fn fails_to_parse_builtin_outside_macro_body() {
    let builtin_funcs = [
        "__codesize",
        "__tablesize",
        "__tablestart",
        "__FUNC_SIG",
        "__EVENT_HASH",
        "__ERROR",
        "__RIGHTPAD",
    ];

    for builtin in builtin_funcs {
        let source = &format!("{}(MAIN)", builtin);
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let mut lexer = Lexer::new(flattened_source.clone());
        assert_eq!(lexer.source, flattened_source);

        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let fn_name_span = Span::new(0..builtin.len(), None);
        assert_eq!(
            unwrapped,
            Token::new(TokenKind::BuiltinFunction(builtin.to_string()), fn_name_span.clone())
        );
        assert_eq!(lexer.current_span().deref(), &fn_name_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // MAIN
        let _ = lexer.next(); // close parenthesis

        // We covered the whole source
        assert_eq!(lexer.current_span().end, source.len());
        assert!(lexer.eof);
    }
}

#[test]
#[should_panic]
fn fails_to_parse_invalid_builtin() {
    let builtin_funcs = ["__not_a_builtin", "__another_not_a_builtin", "__last_not_a_builtin"];

    for builtin in builtin_funcs {
        let source = &format!(
            r#"
            #define macro TEST() = takes(0) returns(0) {}
                {}(MAIN)
            {}
            "#,
            "{", builtin, "}",
        );
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let mut lexer = Lexer::new(flattened_source.clone());
        assert_eq!(lexer.source, flattened_source);

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
        let builtin_span = Span::new(74..74 + builtin.len(), None);
        assert_eq!(
            unwrapped,
            Token::new(TokenKind::BuiltinFunction(builtin.to_string()), builtin_span.clone())
        );
        assert_eq!(lexer.current_span().deref(), &builtin_span);

        let _ = lexer.next(); // open parenthesis
        let _ = lexer.next(); // MAIN
        let _ = lexer.next(); // close parenthesis
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // }
        let _ = lexer.next(); // whitespace

        // We covered the whole source
        assert_eq!(lexer.current_span().end, source.len());
        assert!(lexer.eof);
    }
}
