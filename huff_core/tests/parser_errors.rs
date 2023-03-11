use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn test_invalid_macro_statement() {
    let source = r#"
    #define macro CONSTRUCTOR() = takes(0) returns (0) {}

    #define macro MINT() = takes(0) returns (0) {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]

        FREE_STORAGE_POINTER()
    }

    #define macro MAIN() = takes(0) returns (0) {
        0x00 calldataload 0xE0 shr
        dup1 0x40c10f19 eq mints jumpi

        mints:
            MINT()
    }
    "#;

    let const_start = source.find("FREE_STORAGE_POINTER()").unwrap_or(0);
    let const_end = const_start + "FREE_STORAGE_POINTER()".len() - 1;

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));

    // This should be caught before codegen invalid macro statement
    match parser.parse() {
        Ok(_) => panic!("moose"),
        Err(e) => {
            assert_eq!(
                e,
                ParserError {
                    kind: ParserErrorKind::InvalidTokenInMacroBody(TokenKind::FreeStoragePointer),
                    hint: None,
                    spans: AstSpan(vec![Span { start: const_start, end: const_end, file: None }]),
                }
            )
        }
    }
}

#[test]
fn test_unexpected_type() {
    let source = "#define function func() internal returns ()";

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));

    match parser.parse() {
        Ok(_) => panic!("moose"),
        Err(e) => {
            assert_eq!(
                e,
                ParserError {
                    kind: ParserErrorKind::UnexpectedType(TokenKind::Ident("internal".to_string())),
                    hint: Some(
                        "Expected one of: `view`, `pure`, `payable`, `nonpayable`.".to_string(),
                    ),
                    spans: AstSpan(vec![Span {
                        start: source.find("internal").unwrap_or(0),
                        end: source.find("internal").unwrap_or(0) + "internal".len() - 1,
                        file: None
                    }]),
                }
            )
        }
    }
}

#[test]
fn test_invalid_definition() {
    let source = "#define invalid func() returns ()";

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));

    match parser.parse() {
        Ok(_) => panic!("moose"),
        Err(e) => {
            assert_eq!(
                e,
                ParserError {
                    kind: ParserErrorKind::InvalidDefinition(TokenKind::Ident("invalid".to_string())),
                    hint: Some(
                        "Definition must be one of: `function`, `event`, `constant`, `error`, `macro`, `fn`, or `test`."
                            .to_string()
                    ),
                    spans: AstSpan(vec![Span {
                        start: source.find("invalid").unwrap_or(0),
                        end: source.find("invalid").unwrap_or(0) + "invalid".len() - 1,
                        file: None
                    }]),
                }
            )
        }
    }
}

#[test]
fn test_invalid_constant_value() {
    let invalid_constant_values = vec![
        ("ident", TokenKind::Ident("ident".to_string())),
        ("<", TokenKind::LeftAngle),
        ("{", TokenKind::OpenBrace),
        ("[", TokenKind::OpenBracket),
        ("(", TokenKind::OpenParen),
        (":", TokenKind::Colon),
        (",", TokenKind::Comma),
        ("+", TokenKind::Add),
        ("-", TokenKind::Sub),
    ];

    for (value, kind) in invalid_constant_values {
        let source = &format!("#define constant CONSTANT = {value}");

        let full_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(full_source.source);
        let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
        let mut parser = Parser::new(tokens, Some("".to_string()));

        match parser.parse() {
            Ok(_) => panic!("moose"),
            Err(e) => {
                assert_eq!(
                    e,
                    ParserError {
                        kind: ParserErrorKind::InvalidConstantValue(kind),
                        hint: Some(
                            "Expected constant value to be a literal or `FREE_STORAGE_POINTER()`"
                                .to_string()
                        ),
                        spans: AstSpan(vec![Span {
                            start: source.find(value).unwrap_or(0),
                            end: source.find(value).unwrap_or(0) + value.len() - 1,
                            file: None
                        }]),
                    }
                )
            }
        }
    }
}

#[test]
fn test_invalid_token_in_macro_body() {
    let invalids = vec![
        ("{", TokenKind::OpenBrace),
        ("(", TokenKind::OpenParen),
        (":", TokenKind::Colon),
        (",", TokenKind::Comma),
        ("+", TokenKind::Add),
        ("-", TokenKind::Sub),
        ("/", TokenKind::Div),
    ];

    for (value, kind) in invalids {
        let source = &format!(
            r#"#define macro CONSTANT() = takes (0) returns (0) {{
            {value}
        }}"#
        );

        let full_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(full_source.source);
        let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
        let mut parser = Parser::new(tokens, Some("".to_string()));

        match parser.parse() {
            Ok(_) => panic!("moose"),
            Err(e) => {
                assert_eq!(
                    e,
                    ParserError {
                        kind: ParserErrorKind::InvalidTokenInMacroBody(kind),
                        hint: None,
                        spans: AstSpan(vec![Span {
                            start: source.rfind(value).unwrap_or(0),
                            end: source.rfind(value).unwrap_or(0) + value.len() - 1,
                            file: None
                        }]),
                    }
                )
            }
        }
    }
}

#[test]
fn test_invalid_token_in_label_definition() {
    let invalids = vec![
        ("{", TokenKind::OpenBrace),
        ("(", TokenKind::OpenParen),
        (":", TokenKind::Colon),
        (",", TokenKind::Comma),
        ("+", TokenKind::Add),
        ("-", TokenKind::Sub),
        ("/", TokenKind::Div),
    ];

    for (value, kind) in invalids {
        let source = &format!(
            r#"#define macro CONSTANT() = takes (0) returns (0) {{
            lab:
                {value}
        }}"#
        );

        let full_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(full_source.source);
        let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
        let mut parser = Parser::new(tokens, Some("".to_string()));

        match parser.parse() {
            Ok(_) => panic!("moose"),
            Err(e) => {
                assert_eq!(
                    e,
                    ParserError {
                        kind: ParserErrorKind::InvalidTokenInLabelDefinition(kind),
                        hint: None,
                        spans: AstSpan(vec![Span {
                            start: source.rfind(value).unwrap_or(0),
                            end: source.rfind(value).unwrap_or(0) + value.len() - 1,
                            file: None
                        }]),
                    }
                )
            }
        }
    }
}

#[test]
fn test_invalid_single_arg() {
    for _ in 0..10_000 {
        let random_char = rand::random::<u8>() as char;
        if random_char.is_numeric() ||
            !random_char.is_alphabetic() ||
            random_char.to_string().as_bytes().len() > 1
        {
            continue
        }
        let source = &format!("#define macro CONSTANT() = takes ({random_char}) returns (0) {{}}");

        let full_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(full_source.source);
        let tokens = lexer
            .into_iter()
            .map(|x| match x {
                Ok(t) => t,
                Err(_) => {
                    Token { kind: TokenKind::Add, span: Span { start: 0, end: 0, file: None } }
                }
            })
            .collect::<Vec<Token>>();
        let mut parser = Parser::new(tokens, Some("".to_string()));

        match parser.parse() {
            Ok(_) => panic!("moose"),
            Err(e) => {
                assert_eq!(
                    e,
                    ParserError {
                        kind: ParserErrorKind::InvalidSingleArg(TokenKind::Ident(format!(
                            "{random_char}"
                        ))),
                        hint: Some("Expected number representing stack item count.".to_string()),
                        spans: AstSpan(vec![Span { start: 34, end: 34, file: None }]),
                    }
                )
            }
        }
    }
}
