use huff_lexer::*;
use huff_utils::prelude::*;

#[test]
fn lexes_arg_calls() {
    let source = r#"
    /* Transfer Functions */
    #define macro TRANSFER_TAKE_FROM(error) = takes(3) returns (3) {
        // Ensure that the sender has a sufficient balance.
        // input stack: [value, from, to]
        dup2                // [from, value, from, to]
        [BALANCE_LOCATION] LOAD_ELEMENT_FROM_KEYS(0x00)  // [balance, value, from, to]
        dup1                // [balance, balance, value, from, to]
        dup3                // [value, balance, balance, value, from, to]
        gt                  // [value>balance, balance, value, from, to]
        <error> jumpi       // [balance, value, from, to]

        // Update the sender's balance.
        // input stack: [balance, value, from, to]
        dup2                // [value, balance, value, from, to]
        swap1               // [balance, value, value, from, to]
        sub                 // [balance - value, value, from, to]
        dup3                // [from, balance-value, value, from, to]
        [BALANCE_LOCATION] STORE_ELEMENT_FROM_KEYS(0x00) // [value, from, to]
    }
    "#;

    // Parse tokens
    let mut lexer = Lexer::new(source);
    assert_eq!(lexer.source, source);

    // Eat Tokens
    let _ = lexer.next(); // Whitespace
    let _ = lexer.next(); // /* */ Comment
    let _ = lexer.next(); // #define
    let _ = lexer.next(); // macro keyword
    let _ = lexer.next(); // macro name
    let _ = lexer.next(); // paren
    let _ = lexer.next(); // error keyword
    let _ = lexer.next(); // paren

    // The first token should be a single line comment
    let tok = lexer.next();
    let unwrapped = tok.unwrap().unwrap();
    assert_eq!(unwrapped, Token::new(TokenKind::Comment("// comment contents "), Span::new(0..20)));
    assert_eq!(lexer.span, Span::new(0..20));


    // Get an EOF token
    let tok = lexer.next();
    println!("Lexer got token: {:?}", tok);
    let tok = tok.unwrap().unwrap();
    assert_eq!(tok, Token::new(TokenKind::Eof, Span::new(1..1)));
    assert_eq!(lexer.span, Span::new(1..1));

    // We should have reached EOF now
    assert_eq!(lexer.span.end, source.len());
    assert!(lexer.eof);
    assert!(lexer.next().is_none());

}