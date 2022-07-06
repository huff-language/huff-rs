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
