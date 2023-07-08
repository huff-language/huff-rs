use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn parses_jump_table() {
    let source = "#define jumptable JUMP_TABLE()";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
        .collect::<Vec<Token>>();

    assert_eq!(tokens.get(0).unwrap().kind, TokenKind::Define);
    assert_eq!(tokens.get(1).unwrap().kind, TokenKind::JumpTable);
    assert_eq!(tokens.get(2).unwrap().kind, TokenKind::Ident(String::from("JUMP_TABLE")));
    assert_eq!(tokens.get(3).unwrap().kind, TokenKind::OpenParen);
    assert_eq!(tokens.get(4).unwrap().kind, TokenKind::CloseParen);
}

#[test]
fn parses_packed_jump_table() {
    let source = "#define jumptable__packed JUMP_TABLE_PACKED()";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
        .collect::<Vec<Token>>();

    assert_eq!(tokens.get(0).unwrap().kind, TokenKind::Define);
    assert_eq!(tokens.get(1).unwrap().kind, TokenKind::JumpTablePacked);
    assert_eq!(tokens.get(2).unwrap().kind, TokenKind::Ident(String::from("JUMP_TABLE_PACKED")));
    assert_eq!(tokens.get(3).unwrap().kind, TokenKind::OpenParen);
    assert_eq!(tokens.get(4).unwrap().kind, TokenKind::CloseParen);
}

#[test]
fn parses_code_table() {
    let source = "#define table CODE_TABLE()";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
        .collect::<Vec<Token>>();

    assert_eq!(tokens.get(0).unwrap().kind, TokenKind::Define);
    assert_eq!(tokens.get(1).unwrap().kind, TokenKind::CodeTable);
    assert_eq!(tokens.get(2).unwrap().kind, TokenKind::Ident(String::from("CODE_TABLE")));
    assert_eq!(tokens.get(3).unwrap().kind, TokenKind::OpenParen);
    assert_eq!(tokens.get(4).unwrap().kind, TokenKind::CloseParen);
}
