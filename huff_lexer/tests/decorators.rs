use huff_lexer::Lexer;
use huff_utils::prelude::{str_to_bytes32, FullFileSource, Span, Token, TokenKind};
use std::ops::Deref;

#[test]
fn parses_decorator() {
    let key_words = ["macro", "fn", "test"];

    for s in key_words {
        let source = &format!(
            r#"
            #[calldata(0x01)]
            #define {} NUMS() = takes(0) returns(1) {}
                0x00 dup1 mstore
            {}
            "#,
            s, "{", "}",
        );

        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let mut lexer = Lexer::new(flattened_source);

        let _ = lexer.next(); // whitespace

        // #
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let returns_span = Span::new(13..14, None);
        assert_eq!(unwrapped, Token::new(TokenKind::Pound, returns_span.clone()));
        assert_eq!(lexer.current_span().deref(), &returns_span);

        // [
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let returns_span = Span::new(14..15, None);
        assert_eq!(unwrapped, Token::new(TokenKind::OpenBracket, returns_span.clone()));
        assert_eq!(lexer.current_span().deref(), &returns_span);

        // calldata
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let returns_span = Span::new(15..23, None);
        assert_eq!(
            unwrapped,
            Token::new(TokenKind::Ident(String::from("calldata")), returns_span.clone())
        );
        assert_eq!(lexer.current_span().deref(), &returns_span);

        // (
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let returns_span = Span::new(23..24, None);
        assert_eq!(unwrapped, Token::new(TokenKind::OpenParen, returns_span.clone()));
        assert_eq!(lexer.current_span().deref(), &returns_span);

        // 0x01
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let returns_span = Span::new(26..28, None);
        assert_eq!(
            unwrapped,
            Token::new(TokenKind::Literal(str_to_bytes32("01")), returns_span.clone())
        );
        assert_eq!(lexer.current_span().deref(), &returns_span);

        // )
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let returns_span = Span::new(28..29, None);
        assert_eq!(unwrapped, Token::new(TokenKind::CloseParen, returns_span.clone()));
        assert_eq!(lexer.current_span().deref(), &returns_span);

        // ]
        let tok = lexer.next();
        let unwrapped = tok.unwrap().unwrap();
        let returns_span = Span::new(29..30, None);
        assert_eq!(unwrapped, Token::new(TokenKind::CloseBracket, returns_span.clone()));
        assert_eq!(lexer.current_span().deref(), &returns_span);

        let _ = lexer.next(); // whitespace'
        let _ = lexer.next(); // define
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // body type
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // NUMS
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
        let _ = lexer.next(); // 1
        let _ = lexer.next(); // close parenthesis
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // {
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // 0x00
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // dup1
        let _ = lexer.next(); // whitespace
        let _ = lexer.next(); // mstore
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
fn fails_to_parse_decorator_in_body() {
    let key_words = ["macro", "fn", "test"];

    for s in key_words {
        let source = &format!(
            r#"
            #define {} NUMS() = takes(0) returns(1) {}
                #[calldata(0x01)]
                0x00 dup1 mstore
            {}
            "#,
            s, "{", "}",
        );

        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let mut lexer = Lexer::new(flattened_source);

        for token in lexer.by_ref() {
            if let Err(e) = token {
                panic!("{:?}", e);
            }
        }
    }
}
