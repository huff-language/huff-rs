use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn parse_label() {
    let source =
        "#define macro HELLO_WORLD() = takes(3) returns(0) {\n0x00 mstore\n 0x01 0x02 add cool_label:\n0x01\n}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
        .collect::<Vec<Token>>();

    assert_eq!(
        tokens.get(tokens.len() - 5).unwrap().kind,
        TokenKind::Label("cool_label".to_string())
    );
    assert_eq!(tokens.get(tokens.len() - 4).unwrap().kind, TokenKind::Colon);
}

#[test]
fn parse_label_with_opcode_name() {
    let source =
        "#define macro HELLO_WORLD() = takes(3) returns(0) {\n0x00 mstore\n 0x01 0x02 add cool_label_return_swap1_mload:\n0x01\n}";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
        .collect::<Vec<Token>>();

    assert_eq!(
        tokens.get(tokens.len() - 5).unwrap().kind,
        TokenKind::Label("cool_label_return_swap1_mload".to_string())
    );
    assert_eq!(tokens.get(tokens.len() - 4).unwrap().kind, TokenKind::Colon);
}
