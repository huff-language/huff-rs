use huff_lexer::*;
use huff_parser::*;
use huff_utils::{evm::Opcode, prelude::*};

#[test]
fn not_mistaken_as_opcode() {
    let source = 
    r#"
        #define macro IS_AUTHORIZED(not_authed_arg) = takes(0) returns(0) {}
        #define macro MAIN() = takes(0) returns(0) {
            IS_AUTHORIZED(not_authed)
            not_authed:
                return
        }
    "#;
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);

    let tokens = lexer
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
        .collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let contract = parser.parse().unwrap();
    assert_eq!(
        true,
        true
    );
}