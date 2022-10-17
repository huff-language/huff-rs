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
                    IS_AUTHORIZED({})
                    {}:
                        return
                }}
            "#,
            label, label
        );
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(flattened_source);

        let tokens = lexer
            .into_iter()
            .map(|x| x.unwrap())
            .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
            .collect::<Vec<Token>>();
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
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Should fail here
    parser.parse().unwrap();
}
