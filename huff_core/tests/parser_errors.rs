use huff_codegen::*;
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
    let const_end = const_start + "FREE_STORAGE_POINTER()".len();

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source);
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
    let lexer = Lexer::new(full_source);
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
                        end: source.find("internal").unwrap_or(0) + "internal".len(),
                        file: None
                    }]),
                }
            )
        }
    }
}

#[test]
fn test_invalid_definition() {
    let source = "#define test func() returns ()";

    let full_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(full_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, Some("".to_string()));

    match parser.parse() {
        Ok(_) => panic!("moose"),
        Err(e) => {
            assert_eq!(
                e,
                ParserError {
                    kind: ParserErrorKind::InvalidDefinition(TokenKind::Ident("test".to_string())),
                    hint: Some(
                        "Definition must be one of: `function`, `event`, `constant`, or `macro`."
                            .to_string()
                    ),
                    spans: AstSpan(vec![Span {
                        start: source.find("test").unwrap_or(0),
                        end: source.find("test").unwrap_or(0) + "test".len(),
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
        let source = &format!("#define constant CONSTANT = {}", value);

        let full_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(full_source);
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
                            end: source.find(value).unwrap_or(0) + value.len(),
                            file: None
                        }]),
                    }
                )
            }
        }
    }
}
