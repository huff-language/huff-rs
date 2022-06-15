use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn table_with_no_body() {
    let table_kinds = [TokenKind::JumpTable, TokenKind::JumpTablePacked, TokenKind::CodeTable];

    for kind in table_kinds {
        let source = format!("#define {} TEST_TABLE() = {}{}", kind.to_string(), "{", "}");
        let lexer = Lexer::new(source.as_str());
        let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();

        let mut parser = Parser::new(tokens, None);

        let table_definition = parser.parse().unwrap().tables[0].clone();
        assert_eq!(
            table_definition,
            TableDefinition {
                name: "TEST_TABLE".to_string(),
                kind: TableKind::from(kind),
                statements: vec![],
                size: Literal::default(),
            }
        );
        assert_eq!(parser.current_token.kind, TokenKind::Eof);
    }
}

#[test]
fn table_with_body() {
    // TODO: Code tables are not yet supported
    let table_kinds = [(TokenKind::JumpTable, "96"), (TokenKind::JumpTablePacked, "06")];

    for (kind, expected_size) in table_kinds {
        let source = format!(
            "#define {} TEST_TABLE() = {}\nlabel_call_1 label_call_2 label_call_3\n{}",
            kind.to_string(),
            "{",
            "}"
        );
        let lexer = Lexer::new(source.as_str());
        let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();

        let mut parser = Parser::new(tokens, None);

        let table_definition = parser.parse().unwrap().tables[0].clone();
        assert_eq!(
            table_definition,
            TableDefinition {
                name: "TEST_TABLE".to_string(),
                kind: TableKind::from(kind),
                statements: vec![
                    Statement::LabelCall("label_call_1".to_string()),
                    Statement::LabelCall("label_call_2".to_string()),
                    Statement::LabelCall("label_call_3".to_string()),
                ],
                size: str_to_bytes32(expected_size),
            }
        );
        assert_eq!(parser.current_token.kind, TokenKind::Eof);
    }
}
