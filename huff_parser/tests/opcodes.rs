use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn not_mistaken_as_opcode() {
    for opcode in OPCODES {
        let label = format!("{}_label", (*opcode).to_owned());
        let source = &format!(
            r#"
                #define macro IS_AUTHORIZED(some_arg) = takes(0) returns(0) {{}}
                #define macro MAIN() = takes(0) returns(0) {{
                    IS_AUTHORIZED({label})
                    {label}:
                        return
                }}
            "#
        );
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = lexer::LexerNew::new(flattened_source.source);

        let tokens = lexer
            .into_iter()
            .map(|x| x.unwrap())
            .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
            .collect::<Vec<Token>>();

        dbg!(tokens.clone());

        let actual_label_arg = tokens[tokens.len() - 7].kind.clone();
        let actual_label = tokens[tokens.len() - 5].kind.clone();
        let mut parser = Parser::new(tokens, None);
        // parsing to ensure tokens syntax is valid
        let _contract = parser.parse().unwrap();
        assert_eq!(actual_label_arg, TokenKind::Ident(label.clone()));
        assert_eq!(actual_label, TokenKind::Label(label));
    }
}

#[test]
#[should_panic]
fn test_invalid_push_non_literal() {
    let source: &str = r#"
        // Here we have a macro invocation directly in the parameter list - this should fail
        #define macro MAIN() = takes (0) returns (0) {
            push1 0x10
            push32 0x108
            push1 push1
        }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = lexer::LexerNew::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Should fail here
    parser.parse().unwrap();
}

#[test]
fn test_push_literals() {
    let source: &str = r#"
        #define macro MAIN() = {
            push1 0x10
            push32 0x108
            push1 0x10 0x10
        }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = lexer::LexerNew::new(flattened_source.source);
    // let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();

    let expected_tokens = vec![
        Token { kind: TokenKind::Whitespace, span: Span { start: 0, end: 9, file: None } },
        Token { kind: TokenKind::Define, span: Span { start: 9, end: 16, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 16, end: 17, file: None } },
        Token { kind: TokenKind::Macro, span: Span { start: 17, end: 22, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 22, end: 23, file: None } },
        Token {
            kind: TokenKind::Ident("MAIN".to_string()),
            span: Span { start: 23, end: 27, file: None },
        },
        Token { kind: TokenKind::OpenParen, span: Span { start: 27, end: 28, file: None } },
        Token { kind: TokenKind::CloseParen, span: Span { start: 28, end: 29, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 29, end: 30, file: None } },
        Token { kind: TokenKind::Assign, span: Span { start: 30, end: 31, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 31, end: 32, file: None } },
        Token { kind: TokenKind::OpenBrace, span: Span { start: 32, end: 33, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 33, end: 46, file: None } },
        Token {
            kind: TokenKind::Opcode(Opcode::Push1),
            span: Span { start: 46, end: 51, file: None },
        },
        Token { kind: TokenKind::Whitespace, span: Span { start: 51, end: 52, file: None } },
        Token {
            kind: TokenKind::Literal([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 16,
            ]),
            span: Span { start: 54, end: 56, file: None },
        },
        Token { kind: TokenKind::Whitespace, span: Span { start: 56, end: 69, file: None } },
        Token {
            kind: TokenKind::Opcode(Opcode::Push32),
            span: Span { start: 69, end: 75, file: None },
        },
        Token { kind: TokenKind::Whitespace, span: Span { start: 75, end: 76, file: None } },
        Token {
            kind: TokenKind::Literal([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 1, 8,
            ]),
            span: Span { start: 78, end: 81, file: None },
        },
        Token { kind: TokenKind::Whitespace, span: Span { start: 81, end: 94, file: None } },
        Token {
            kind: TokenKind::Opcode(Opcode::Push1),
            span: Span { start: 94, end: 99, file: None },
        },
        Token { kind: TokenKind::Whitespace, span: Span { start: 99, end: 100, file: None } },
        Token {
            kind: TokenKind::Literal([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 16,
            ]),
            span: Span { start: 102, end: 104, file: None },
        },
        Token { kind: TokenKind::Whitespace, span: Span { start: 104, end: 105, file: None } },
        Token {
            kind: TokenKind::Literal([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 16,
            ]),
            span: Span { start: 107, end: 109, file: None },
        },
        Token { kind: TokenKind::Whitespace, span: Span { start: 109, end: 118, file: None } },
        Token { kind: TokenKind::CloseBrace, span: Span { start: 118, end: 119, file: None } },
        Token { kind: TokenKind::Whitespace, span: Span { start: 119, end: 124, file: None } },
        Token { kind: TokenKind::Eof, span: Span { start: 124, end: 124, file: None } },
    ];
    assert_eq!(expected_tokens, tokens);

    // This should parse correctly
    let mut parser = Parser::new(tokens, None);
    parser.parse().unwrap();
}
