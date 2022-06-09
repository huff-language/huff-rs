use huff_lexer::*;
use huff_parser::*;
use huff_utils::{evm::Opcode, prelude::*};

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
                Statement::Literal(str_to_bytes32("00")),
                Statement::Opcode(Opcode::Mstore),
                Statement::Literal(str_to_bytes32("01")),
                Statement::Literal(str_to_bytes32("02")),
                Statement::Opcode(Opcode::Add)
            ],
            takes: 3,
            returns: 0,
        }
    );
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}

#[test]
fn macro_with_arg_calls() {
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
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens);

    // Grab the first macro
    let macro_definition = parser.parse().unwrap().macros[0].clone();
    assert_eq!(
        macro_definition,
        MacroDefinition {
            name: "TRANSFER_TAKE_FROM".to_string(),
            parameters: vec![Argument {
                arg_type: None,
                name: Some("error".to_string()),
                indexed: false
            }],
            statements: vec![
                Statement::Opcode(Opcode::Dup2),
                Statement::Constant("BALANCE_LOCATION"),
                Statement::MacroInvocation(MacroInvocation {
                    macro_name: "LOAD_ELEMENT_FROM_KEYS".to_string(),
                    args: vec![MacroArg::Literal(str_to_bytes32("00"))]
                }),
                Statement::Opcode(Opcode::Dup1),
                Statement::Opcode(Opcode::Dup3),
                Statement::Opcode(Opcode::Gt),
                Statement::ArgCall("error"),
                Statement::Opcode(Opcode::Jumpi),
                Statement::Opcode(Opcode::Dup2),
                Statement::Opcode(Opcode::Swap1),
                Statement::Opcode(Opcode::Sub),
                Statement::Opcode(Opcode::Dup3),
                Statement::Constant("BALANCE_LOCATION"),
                Statement::MacroInvocation(MacroInvocation {
                    macro_name: "STORE_ELEMENT_FROM_KEYS".to_string(),
                    args: vec![MacroArg::Literal(str_to_bytes32("00"))]
                })
            ],
            takes: 3,
            returns: 3
        }
    );
    assert_eq!(parser.current_token.kind, TokenKind::Eof);
}
