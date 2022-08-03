use huff_codegen::Codegen;
use huff_lexer::Lexer;
use huff_parser::Parser;
use huff_utils::prelude::*;
use std::str::FromStr;

#[test]
fn test_opcode_macro_args() {
    let source = r#"
        #define macro RETURN1(zero) = takes(1) returns(0) {
            <zero> mstore
            0x20 <zero> return
        }

        #define macro MAIN() = takes(0) returns(0) {
            RETURN1(returndatasize)
        }
    "#;

    // Lex + Parse
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    // Create main and constructor bytecode
    let main_bytecode = Codegen::generate_main_bytecode(&contract).unwrap();

    // Full expected bytecode output (generated from huffc) (placed here as a reference)
    let expected_bytecode = "60088060093d393df360ff3d5260203df3";

    // Create bytecode
    let bytecode = format!("60088060093d393df360ff{}", main_bytecode);

    // Check the bytecode
    assert_eq!(bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}

#[test]
fn test_all_opcodes_in_macro_args() {
    for o in OPCODES {
        let source = format!(
            r#"
            #define macro RETURN1(zero) = takes(1) returns(0) {{
                <zero>
            }}

            #define macro MAIN() = takes(0) returns(0) {{
                RETURN1({})
            }}
        "#,
            o
        );

        // Lex + Parse
        let flattened_source = FullFileSource { source: &source, file: None, spans: vec![] };
        let lexer = Lexer::new(flattened_source);
        let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
        let mut parser = Parser::new(tokens, None);
        let mut contract = parser.parse().unwrap();
        contract.derive_storage_pointers();

        // Create main and constructor bytecode
        let main_bytecode = Codegen::generate_main_bytecode(&contract).unwrap();

        // Full expected bytecode output (generated from huffc) (placed here as a reference)
        let expected_bytecode = format!("60088060093d393df360ff{}", Opcode::from_str(o).unwrap());

        // Create bytecode
        let bytecode = format!("60088060093d393df360ff{}", main_bytecode);

        // Check the bytecode
        assert_eq!(bytecode.to_lowercase(), expected_bytecode.to_lowercase());
    }
}
