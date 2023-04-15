use huff_lexer::*;
use huff_utils::{evm::Opcode, prelude::*};

#[test]
fn lexes_arg_calls() {
    let source = r#"
    #define macro TRANSFER_TAKE_FROM(error) = takes(3) returns (3) {
        dup2
        [BALANCE_LOCATION] LOAD_ELEMENT_FROM_KEYS(0x00)
        dup1
        dup3
        gt
        <error> jumpi
    }
    "#;
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };

    // Parse tokens
    let mut lexer = Lexer::new(flattened_source.source);

    // Eat Tokens
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // #define
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // macro keyword
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // macro name
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // error keyword
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // equals
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // takes
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // 3
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // returns
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // 3
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // open brace
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // dup2
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // bracket
    let _ = lexer.next(); // balance pointer
    let _ = lexer.next(); // bracket
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // func
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // Literal
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // dup1
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // dup3
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // gt
    let _ = lexer.next(); // Whitespace

    // We should find a left angle
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::LeftAngle, Span::new(184..184, None)));

    // The we should have an Ident
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Ident("error".to_string()), Span::new(185..189, None)));

    // Then should find a right angle
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::RightAngle, Span::new(190..190, None)));

    let _ = lexer.next(); // Whitespace

    // Jumpi Opcode
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Opcode(Opcode::Jumpi), Span::new(192..196, None)));

    // Eat the rest of the tokens
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // closing brace
    let _ = lexer.next(); // Whitespace

    // Get an EOF token
    let tok = lexer.next().unwrap().unwrap();
    assert_eq!(
        tok,
        Token::new(TokenKind::Eof, Span::new(source.len() - 1..source.len() - 1, None))
    );

    // We should have reached EOF now
    assert!(lexer.eof);
    assert!(lexer.next().is_none());
}
