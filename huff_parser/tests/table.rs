use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
fn table_with_no_body() {
    let table_kinds = [TokenKind::JumpTable, TokenKind::JumpTablePacked, TokenKind::CodeTable];

    for kind in table_kinds {
        let source = &format!("#define {kind} TEST_TABLE() = {}{}", "{", "}");
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };
        let lexer = Lexer::new(flattened_source.source);
        let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();

        let mut parser = Parser::new(tokens, None);

        let kind_offset = kind.to_string().len() + 8;

        let table_definition = parser.parse().unwrap().tables[0].clone();
        assert_eq!(
            table_definition,
            TableDefinition {
                name: "TEST_TABLE".to_string(),
                kind: TableKind::from(kind),
                statements: vec![],
                size: Literal::default(),
                span: AstSpan(vec![
                    Span { start: 0, end: 6, file: None },
                    Span { start: 8, end: kind_offset - 1, file: None },
                    Span { start: kind_offset + 1, end: kind_offset + 10, file: None },
                    Span { start: kind_offset + 11, end: kind_offset + 11, file: None },
                    Span { start: kind_offset + 12, end: kind_offset + 12, file: None },
                    Span { start: kind_offset + 14, end: kind_offset + 14, file: None },
                    Span { start: kind_offset + 16, end: kind_offset + 16, file: None },
                    Span { start: kind_offset + 17, end: kind_offset + 17, file: None }
                ]),
            }
        );
        assert_eq!(parser.current_token.kind, TokenKind::Eof);
    }
}

#[test]
fn table_with_body() {
    // TODO: Code tables are not yet supported
    let table_kinds = [(TokenKind::JumpTable, "60"), (TokenKind::JumpTablePacked, "06")];

    for (kind, expected_size) in table_kinds {
        let source = &format!(
            "#define {} TEST_TABLE() = {}\nlabel_call_1 label_call_2 label_call_3\n{}",
            kind, "{", "}"
        );
        let lb1_start = source.find("label_call_1").unwrap_or(0);
        let lb2_start = source.find("label_call_2").unwrap_or(0);
        let lb3_start = source.find("label_call_3").unwrap_or(0);
        let flattened_source = FullFileSource { source, file: None, spans: vec![] };

        let lexer = Lexer::new(flattened_source.source);
        let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();

        let mut parser = Parser::new(tokens, None);

        let kind_offset = kind.to_string().len() + 8;

        let table_definition = parser.parse().unwrap().tables[0].clone();
        assert_eq!(
            table_definition,
            TableDefinition {
                name: "TEST_TABLE".to_string(),
                kind: TableKind::from(kind),
                statements: vec![
                    Statement {
                        ty: StatementType::LabelCall("label_call_1".to_string()),
                        span: AstSpan(vec![Span {
                            start: lb1_start,
                            end: lb1_start + "label_call_1".len() - 1,
                            file: None
                        }]),
                    },
                    Statement {
                        ty: StatementType::LabelCall("label_call_2".to_string()),
                        span: AstSpan(vec![Span {
                            start: lb2_start,
                            end: lb2_start + "label_call_2".len() - 1,
                            file: None
                        }]),
                    },
                    Statement {
                        ty: StatementType::LabelCall("label_call_3".to_string()),
                        span: AstSpan(vec![Span {
                            start: lb3_start,
                            end: lb3_start + "label_call_3".len() - 1,
                            file: None
                        }]),
                    },
                ],
                size: str_to_bytes32(expected_size),
                span: AstSpan(vec![
                    Span { start: 0, end: 6, file: None },
                    Span { start: 8, end: kind_offset - 1, file: None },
                    Span { start: kind_offset + 1, end: kind_offset + 10, file: None },
                    Span { start: kind_offset + 11, end: kind_offset + 11, file: None },
                    Span { start: kind_offset + 12, end: kind_offset + 12, file: None },
                    Span { start: kind_offset + 14, end: kind_offset + 14, file: None },
                    Span { start: kind_offset + 16, end: kind_offset + 16, file: None },
                    Span { start: kind_offset + 18, end: kind_offset + 29, file: None },
                    Span { start: kind_offset + 31, end: kind_offset + 42, file: None },
                    Span { start: kind_offset + 44, end: kind_offset + 55, file: None },
                    Span { start: kind_offset + 57, end: kind_offset + 57, file: None }
                ]),
            }
        );
        assert_eq!(parser.current_token.kind, TokenKind::Eof);
    }
}
