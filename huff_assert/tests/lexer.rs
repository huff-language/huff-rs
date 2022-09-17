use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn test_parse_assertions() {
    let source = "#define macro TEST() = takes (0) returns (0) {\
        0x10        $ [val]
        pop         $ []
    }";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| matches!(x.kind, TokenKind::Stack(_)))
        .collect::<Vec<Token>>();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens.get(0).unwrap().kind, TokenKind::Stack(vec!["val".to_string()]));
    assert_eq!(tokens.get(1).unwrap().kind, TokenKind::Stack(vec![]));
}

#[test]
fn test_parse_comments_in_assertions() {
    let source = "#define macro TEST() = takes(0) returns(0) {\
        0x10        $ [val]
        0x20        $ [/*oth_val, */ oth_val, val]
        pop         $ [val]
        pop         $ []
    }";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| matches!(x.kind, TokenKind::Stack(_)))
        .collect::<Vec<Token>>();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens.get(0).unwrap().kind, TokenKind::Stack(vec!["val".to_string()]));
    assert_eq!(
        tokens.get(1).unwrap().kind,
        TokenKind::Stack(vec!["oth_val".to_string(), "val".to_string()])
    );
    assert_eq!(tokens.get(2).unwrap().kind, TokenKind::Stack(vec!["val".to_string()]));
    assert_eq!(tokens.get(3).unwrap().kind, TokenKind::Stack(vec![]));
}

#[test]
fn test_parse_nested_comments_in_assertions() {
    let source = "#define macro TEST() = takes(0) returns(0) {\
        0x10        $ [/*oth_val,/* stuff*/ */ val]
        pop         $ []
    }";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| matches!(x.kind, TokenKind::Stack(_)))
        .collect::<Vec<Token>>();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens.get(0).unwrap().kind, TokenKind::Stack(vec!["val".to_string()]));
    assert_eq!(tokens.get(1).unwrap().kind, TokenKind::Stack(vec![]));
}

#[test]
fn test_parse_multiple_comments_in_assertions() {
    let source = "#define macro TEST() = takes(0) returns(0) {\
        0x10        $ [/*oth_val,*/ val /* stuff*/ ]
        pop         $ []
    }";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| matches!(x.kind, TokenKind::Stack(_)))
        .collect::<Vec<Token>>();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens.get(0).unwrap().kind, TokenKind::Stack(vec!["val".to_string()]));
    assert_eq!(tokens.get(1).unwrap().kind, TokenKind::Stack(vec![]));
}
