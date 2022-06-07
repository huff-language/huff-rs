use huff_lexer::*;
use huff_parser::*;
use huff_utils::{evm::Opcode, prelude::*};

mod common;
use common::*;

#[test]
fn empty_macro() {
    let source = "#define macro HELLO_WORLD() = takes(0) returns(4) {}";
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    assert_eq!(
        macro_definition,
        MacroDefinition {
            name: "HELLO_WORLD".to_string(),
            parameters: vec![],
            statements: vec![],
            takes: 0,
            returns: 4,
        }
    );
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn macro_with_simple_body() {
    let source =
        "#define macro HELLO_WORLD() = takes(3) returns(0) {\n0x00 mstore\n 0x01 0x02 add\n}";
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    assert_eq!(
        macro_definition,
        MacroDefinition {
            name: "HELLO_WORLD".to_string(),
            parameters: vec![],
            statements: vec![
                Statement::Literal(create_literal_from_str("00")),
                Statement::Opcode(Opcode::Mstore),
                Statement::Literal(create_literal_from_str("01")),
                Statement::Literal(create_literal_from_str("02")),
                Statement::Opcode(Opcode::Add)
            ],
            takes: 3,
            returns: 0,
        }
    );
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}
