use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn lexes_assign_op() {
    let source = "#define constant TRANSFER_EVENT_SIGNATURE =";
    // comment contents \n#define macro HELLO_WORLD()";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // This token should be a Define identifier
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(0..7);
    assert_eq!(unwrapped, Token::new(TokenKind::Define, define_span));
    assert_eq!(lexer.span, define_span);

    // The next token should be the whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let define_span = Span::new(7..8);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, define_span));
    assert_eq!(lexer.span, define_span);

    // Then we should parse the constant keyword
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let constant_span = Span::new(8..16);
    assert_eq!(unwrapped, Token::new(TokenKind::Constant, constant_span));
    assert_eq!(lexer.span, constant_span);

    // The next token should be another whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let ws_span = Span::new(16..17);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, ws_span));
    assert_eq!(lexer.span, ws_span);

    // Then we should get the function name
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let function_span = Span::new(17..41);
    assert_eq!(unwrapped, Token::new(TokenKind::Ident("TRANSFER_EVENT_SIGNATURE"), function_span));
    assert_eq!(lexer.span, function_span);

    // Then we should have another whitespace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let whitespace_span = Span::new(41..42);
    assert_eq!(unwrapped, Token::new(TokenKind::Whitespace, whitespace_span));
    assert_eq!(lexer.span, whitespace_span);

    // Finally, we have our assign operator
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let assign_span = Span::new(42..43);
    assert_eq!(unwrapped, Token::new(TokenKind::Assign, assign_span));
    assert_eq!(lexer.span, assign_span);

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}

#[test]
fn lexes_brackets() {
    let source = "[TOTAL_SUPPLY_LOCATION] sload";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // This token should be the open bracket
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let bracket_span = Span::new(0..1);
    assert_eq!(unwrapped, Token::new(TokenKind::OpenBracket, bracket_span));
    assert_eq!(lexer.span, bracket_span);

    // The next token should be the location identifier
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let loc_span = Span::new(1..22);
    assert_eq!(unwrapped, Token::new(TokenKind::Ident("TOTAL_SUPPLY_LOCATION"), loc_span));
    assert_eq!(lexer.span, loc_span);

    // Then we should parse the closing bracket
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let bracket_span = Span::new(22..23);
    assert_eq!(unwrapped, Token::new(TokenKind::CloseBracket, bracket_span));
    assert_eq!(lexer.span, bracket_span);

    // Eat the last tokens
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // sload opcode

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}

#[test]
fn lexes_braces() {
    // Uh-oh, scary source code  ( ._. )
    let source = r#"
#define macro CONSTRUCTOR() = takes(0) returns(0) {
    // Set msg.sender as the owner of the contract.
    OWNABLE_CONSTRUCTOR()
}
    "#;

    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Eat the non-brace tokens
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // define
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // macro
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // CONSTRUCTOR
    let _ = lexer.next(); // open paren
    let _ = lexer.next(); // close paren
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // assign
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // takes keyword
    let _ = lexer.next(); // open paren
    let _ = lexer.next(); // number
    let _ = lexer.next(); // close paren
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // returns keyword
    let _ = lexer.next(); // open paren
    let _ = lexer.next(); // number
    let _ = lexer.next(); // close paren
    let _ = lexer.next(); // whitespace

    // This token should be the open brace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let brace_span = Span::new(51..52);
    assert_eq!(unwrapped, Token::new(TokenKind::OpenBrace, brace_span));
    assert_eq!(lexer.span, brace_span);

    // Eat the characters in between braces
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // comment
    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // ident
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // whitespace

    // We should now have the closing brace
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let brace_span = Span::new(131..132);
    assert_eq!(unwrapped, Token::new(TokenKind::CloseBrace, brace_span));
    assert_eq!(lexer.span, brace_span);

    // Eat the last whitespace
    let _ = lexer.next(); // whitespace

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}

#[test]
fn lexes_math_ops() {
    // MATHS
    let source = r#"100 + 10 - 20 * 5 / 4"#;
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Eat the number and whitespace
    let _ = lexer.next();
    let _ = lexer.next();

    // This token should be an addition
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let add_span = Span::new(4..5);
    assert_eq!(unwrapped, Token::new(TokenKind::Add, add_span));
    assert_eq!(lexer.span, add_span);

    // Eat the number and whitespaces
    let _ = lexer.next();
    let _ = lexer.next();
    let _ = lexer.next();

    // This token should be a subtraction
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let sub_span = Span::new(9..10);
    assert_eq!(unwrapped, Token::new(TokenKind::Sub, sub_span));
    assert_eq!(lexer.span, sub_span);

    // Eat the number and whitespaces
    let _ = lexer.next();
    let _ = lexer.next();
    let _ = lexer.next();

    // This token should be a multiplication
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let mul_span = Span::new(14..15);
    assert_eq!(unwrapped, Token::new(TokenKind::Mul, mul_span));
    assert_eq!(lexer.span, mul_span);

    // Eat the number and whitespace
    let _ = lexer.next();
    let _ = lexer.next();
    let _ = lexer.next();

    // This token should be a division
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let div_span = Span::new(18..19);
    assert_eq!(unwrapped, Token::new(TokenKind::Div, div_span));
    assert_eq!(lexer.span, div_span);

    // Eat the number and whitespace
    let _ = lexer.next();
    let _ = lexer.next();

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}

#[test]
fn lexes_commas() {
    let source = "test,test";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Eat alphanumerics
    let _ = lexer.next();

    // This token should be the comma
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let comma_span = Span::new(4..5);
    assert_eq!(unwrapped, Token::new(TokenKind::Comma, comma_span));
    assert_eq!(lexer.span, comma_span);

    // Eat alphanumerics
    let _ = lexer.next();

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}

#[test]
fn lexes_comma_sparse() {
    let source = "test , test";
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    let _ = lexer.next(); // alphanumerics
    let _ = lexer.next(); // whitespace

    // This token should be the comma
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    let comma_span = Span::new(5..6);
    assert_eq!(unwrapped, Token::new(TokenKind::Comma, comma_span));
    assert_eq!(lexer.span, comma_span);

    let _ = lexer.next(); // whitespace
    let _ = lexer.next(); // alphanumerics

    // We covered the whole source
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
}
