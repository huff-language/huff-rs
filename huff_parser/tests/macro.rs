use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

fn lex_and_parse(source: &str) -> Result<(), ParserError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[test]
fn empty_macro() {
    let source = "#define macro HELLO_WORLD() = takes(0) returns(0) {}";
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    parser.parse();
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn macro_with_simple_body() {
    let source =
        "#define macro HELLO_WORLD() = takes(0) returns(0) {\n0x00 mstore\n 0x01 0x02 add\n}";
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);
    parser.parse();
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}
