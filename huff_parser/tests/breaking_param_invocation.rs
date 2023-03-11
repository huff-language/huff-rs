use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

#[test]
#[should_panic]
fn test_breaking_param_invocation() {
    let source: &str = r#"
        // Here we have a macro invocation directly in the parameter list - this should fail
        #define macro TEST(<value>) = takes(0) returns(0) {
            <value> 0x00 mstore
            0x20 0x00 return
        }

        #define macro MAIN() = takes(0) returns(0) {
            TEST(0x01)
        }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Should fail here
    parser.parse().unwrap();
}

#[test]
#[should_panic]
fn test_breaking_param_commas() {
    let source: &str = r#"
        // Here we have a macro invocation directly in the parameter list - this should fail
        #define macro TEST(t,,) = takes(0) returns(0) {
            <value> 0x00 mstore
            0x20 0x00 return
        }

        #define macro MAIN() = takes(0) returns(0) {
            TEST(0x01)
        }
    "#;

    // Parse tokens
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);

    // Should fail here
    parser.parse().unwrap();
}
