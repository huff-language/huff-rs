use huff_lexer::Lexer;
use huff_utils::prelude::*;

#[test]
fn single_lex_imports() {
    let import_str = "../huff-examples/erc20/contracts/utils/Ownable.huff";
    let source = format!("#include \"{import_str}\"");
    let lexed_imports = Lexer::lex_imports(&source);
    assert_eq!(lexed_imports.len(), 1);
    assert_eq!(lexed_imports[0], import_str);
}

#[test]
fn commented_lex_imports() {
    let import_str = "../huff-examples/erc20/contracts/utils/Ownable.huff";
    let source = format!(
        r#"
    // #include "{import_str}"
    /* #include "{import_str}" */
    /* test test test */
    #define macro ()
    #include "{import_str}"
    "#
    );

    let lexed_imports = Lexer::lex_imports(&source);
    assert_eq!(lexed_imports.len(), 1);
    assert_eq!(lexed_imports[0], import_str);
}

#[test]
fn multiple_lex_imports() {
    let import_str = "../huff-examples/erc20/contracts/utils/Ownable.huff";
    let source = format!(
        r#"
    #include "{import_str}"
    #include "{import_str}"
    /* test test test */
    #define macro ()
    #include "{import_str}"
    "#
    );

    let lexed_imports = Lexer::lex_imports(&source);
    assert_eq!(lexed_imports.len(), 3);
    for i in lexed_imports {
        assert_eq!(i, import_str);
    }
}

#[test]
fn multiple_lex_imports_single_quotes() {
    let import_str = "../huff-examples/erc20/contracts/utils/Ownable.huff";
    let source = format!(
        r#"
    #include '{import_str}'
    #include '{import_str}'
    "#
    );

    let lexed_imports = Lexer::lex_imports(&source);
    assert_eq!(lexed_imports.len(), 2);
    for i in lexed_imports {
        assert_eq!(i, import_str);
    }
}

#[test]
fn lex_imports_no_ending_quote() {
    let import_str = "../huff-examples/erc20/contracts/utils/Ownable.huff";
    let source = format!("#include '{import_str}");
    let lexed_imports = Lexer::lex_imports(&source);
    assert_eq!(lexed_imports.len(), 0);
}

#[test]
fn lex_imports_no_starting_quote() {
    let import_str = "../huff-examples/erc20/contracts/utils/Ownable.huff";
    let source = format!("#include {import_str}'");
    let lexed_imports = Lexer::lex_imports(&source);
    assert_eq!(lexed_imports.len(), 0);
}

#[test]
fn lex_imports_empty_quotes() {
    // let import_str = "../huff-examples/erc20/contracts/utils/Ownable.huff";
    let source = "#include ''";
    let lexed_imports = Lexer::lex_imports(source);
    assert_eq!(lexed_imports.len(), 1);
    assert_eq!(lexed_imports[0], "");
}

#[test]
fn include_no_quotes() {
    let source = "#include";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source.source);

    // The first token should be a single line comment
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    assert_eq!(unwrapped, Token::new(TokenKind::Include, Span::new(0..7, None)));
    lexer.next();
    assert!(lexer.eof);
}

#[test]
fn include_with_string() {
    let source = "#include \"../huff-examples/erc20/contracts/utils/Ownable.huff\"";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source.source);

    // The first token should be a single line comment
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    assert_eq!(unwrapped, Token::new(TokenKind::Include, Span::new(0..7, None)));

    // Lex the whitespace char
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let literal_span = Span::new(8..8, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, literal_span));

    // Then we should parse the string literal
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let literal_span = Span::new(9..61, None);
    assert_eq!(
        unwrapped,
        Token::new(
            TokenKind::Str("../huff-examples/erc20/contracts/utils/Ownable.huff".to_string()),
            literal_span
        )
    );

    lexer.next();

    // We should have reached EOF now
    assert!(lexer.eof);
}

#[test]
fn include_with_string_single_quote() {
    let source = "#include '../huff-examples/erc20/contracts/utils/Ownable.huff'";
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let mut lexer = Lexer::new(flattened_source.source);

    // The first token should be a single line comment
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    assert_eq!(unwrapped, Token::new(TokenKind::Include, Span::new(0..7, None)));

    // Lex the whitespace char
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let literal_span = Span::new(8..8, None);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, literal_span));

    // Then we should parse the string literal
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let literal_span = Span::new(9..61, None);
    assert_eq!(
        unwrapped,
        Token::new(
            TokenKind::Str("../huff-examples/erc20/contracts/utils/Ownable.huff".to_string()),
            literal_span
        )
    );

    lexer.next();

    // We should have reached EOF now
    assert!(lexer.eof);
}
