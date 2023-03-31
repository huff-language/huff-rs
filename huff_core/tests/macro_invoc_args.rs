use huff_codegen::Codegen;
use huff_lexer::Lexer;
use huff_parser::Parser;
use huff_utils::prelude::*;
use std::str::FromStr;

#[test]
fn test_opcode_macro_args() {
    let source = r#"
        #define macro RETURN1(zero) = takes(0) returns(0) {
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
    let main_bytecode = Codegen::generate_main_bytecode(&contract, None).unwrap();

    // Full expected bytecode output (generated from huffc) (placed here as a reference)
    let expected_bytecode = "60088060093d393df360ff3d5260203df3";

    // Create bytecode
    let bytecode = format!("60088060093d393df360ff{main_bytecode}");

    // Check the bytecode
    assert_eq!(bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}

#[test]
fn test_all_opcodes_in_macro_args() {
    for o in OPCODES {
        let source = format!(
            r#"
            #define macro RETURN1(zero) = takes(0) returns(0) {{
                <zero>
            }}

            #define macro MAIN() = takes(0) returns(0) {{
                RETURN1({o})
            }}
        "#
        );

        // Lex + Parse
        let flattened_source = FullFileSource { source: &source, file: None, spans: vec![] };
        let lexer = Lexer::new(flattened_source);
        let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
        let mut parser = Parser::new(tokens, None);
        let mut contract = parser.parse().unwrap();
        contract.derive_storage_pointers();

        // Create main and constructor bytecode
        let main_bytecode = Codegen::generate_main_bytecode(&contract, None).unwrap();

        // Full expected bytecode output (generated from huffc) (placed here as a reference)
        let expected_bytecode = format!("60088060093d393df360ff{}", Opcode::from_str(o).unwrap());

        // Create bytecode
        let bytecode = format!("60088060093d393df360ff{main_bytecode}");

        // Check the bytecode
        assert_eq!(bytecode.to_lowercase(), expected_bytecode.to_lowercase());
    }
}

#[test]
fn test_constant_macro_arg() {
    let source = r#"
            #define constant A = 0x02

            #define macro RETURN1(zero) = takes(0) returns(0) {
                <zero>
            }

            #define macro MAIN() = takes(0) returns(0) {
                RETURN1(A)
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
    let main_bytecode = Codegen::generate_main_bytecode(&contract, None).unwrap();

    // Full expected bytecode output (generated from huffc) (placed here as a reference)
    let expected_bytecode = "60088060093d393df360ff6002";

    // Create bytecode
    let bytecode = format!("60088060093d393df360ff{main_bytecode}");

    // Check the bytecode
    assert_eq!(bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}

#[test]
fn test_bubbled_label_call_macro_arg() {
    let source = r#"
            #define macro MACRO_A(zero) = takes(0) returns(0) {
                MACRO_B(<zero>)
            }

            #define macro MACRO_B(zero) = takes(0) returns(0) {
                <zero>
            }

            #define macro MAIN() = takes(0) returns(0) {
                label:
                    MACRO_A(label)
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
    let main_bytecode = Codegen::generate_main_bytecode(&contract, None).unwrap();

    // Full expected bytecode output (generated from huffc) (placed here as a reference)
    let expected_bytecode = "60088060093d393df360ff5b610000";

    // Create bytecode
    let bytecode = format!("60088060093d393df360ff{main_bytecode}");

    // Check the bytecode
    assert_eq!(bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}

#[test]
fn test_bubbled_literal_macro_arg() {
    let source = r#"
            #define macro MACRO_A(zero) = takes(0) returns(0) {
                MACRO_B(<zero>)
            }

            #define macro MACRO_B(zero) = takes(0) returns(0) {
                <zero>
            }

            #define macro MAIN() = takes(0) returns(0) {
                MACRO_A(0x420)
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
    let main_bytecode = Codegen::generate_main_bytecode(&contract, None).unwrap();

    // Full expected bytecode output (generated from huffc) (placed here as a reference)
    let expected_bytecode = "60088060093d393df360ff610420";

    // Create bytecode
    let bytecode = format!("60088060093d393df360ff{main_bytecode}");

    // Check the bytecode
    assert_eq!(bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}

#[test]
fn test_bubbled_opcode_macro_arg() {
    let source = r#"
            #define macro MACRO_A(zero) = takes(0) returns(0) {
                MACRO_B(<zero>)
            }

            #define macro MACRO_B(zero) = takes(0) returns(0) {
                <zero>
            }

            #define macro MAIN() = takes(0) returns(0) {
                MACRO_A(returndatasize)
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
    let main_bytecode = Codegen::generate_main_bytecode(&contract, None).unwrap();

    // Full expected bytecode output (generated from huffc) (placed here as a reference)
    let expected_bytecode = "60088060093d393df360ff3d";

    // Create bytecode
    let bytecode = format!("60088060093d393df360ff{main_bytecode}");

    // Check the bytecode
    assert_eq!(bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}

#[test]
fn test_bubbled_constant_macro_arg() {
    let source = r#"
            #define constant A = 0x02

            #define macro MACRO_A(zero) = takes(0) returns(0) {
                MACRO_B(<zero>)
            }

            #define macro MACRO_B(zero) = takes(0) returns(0) {
                <zero>
            }

            #define macro MAIN() = takes(0) returns(0) {
                MACRO_A(A)
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
    let main_bytecode = Codegen::generate_main_bytecode(&contract, None).unwrap();

    // Full expected bytecode output (generated from huffc) (placed here as a reference)
    let expected_bytecode = "60088060093d393df360ff6002";

    // Create bytecode
    let bytecode = format!("60088060093d393df360ff{main_bytecode}");

    // Check the bytecode
    assert_eq!(bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}


#[test]
fn test_bubbled_arg_with_different_name() {
  let source = r#"
          #define macro MACRO_A(arg_a) = takes(0) returns(0) {
            <arg_a>
          }
          #define macro MACRO_B(arg_b) =takes(0) returns(0) {
            MACRO_A(<arg_b>)
          }
          #define macro MAIN() = takes(0) returns(0){
            MACRO_B(0x01)
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
  let main_bytecode = Codegen::generate_main_bytecode(&contract, None).unwrap();

  // Full expected bytecode output (generated from huffc) (placed here as a reference)
  let expected_bytecode = "6001";

  // Check the bytecode
  assert_eq!(main_bytecode, expected_bytecode);

}

#[test]
fn test_macro_macro_arg() {
    let source = r#"
            #define constant TWO = 0x02

            #define macro MUL_BY_10() = takes(1) returns (1) {
              0x0a mul
            }
            
            #define macro EXEC_WITH_VALUE(value, macro) = takes(0) returns(1) {
              <value> <macro>
            }
            
            #define macro MAIN() = takes(0) returns(0) {
                EXEC_WITH_VALUE(TWO, MUL_BY_10)
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
    let main_bytecode = Codegen::generate_main_bytecode(&contract, None).unwrap();

    // Full expected bytecode output (generated from huffc) (placed here as a reference)
    let expected_bytecode = "6002600a02";

    // Check the bytecode
    assert_eq!(main_bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}

#[test]
fn test_bubbled_macro_macro_arg() {
    let source = r#"
        #define constant TWO = 0x02

        #define macro MUL_BY_10() = takes(1) returns (1) {
          0x0a mul
        }
        
        #define macro DO_OP(op) = takes(0) returns(0) {
          <op>
        }
        
        #define macro DIV_BY_5() = takes(1) returns (1) {
          0x05 swap1 DO_OP(div)
        }
        
        #define macro EXEC_WITH_VALUE(value, macro) = takes(0) returns(1) {
          <value> <macro>
        }
        
        #define macro SUM_RESULTS(value, macro1, macro2) = takes(0) returns (1) {
          EXEC_WITH_VALUE(<value>, <macro1>)
          EXEC_WITH_VALUE(<value>, <macro2>)
          add
        }
        
        #define macro MAIN() = takes(0) returns(0) {
          SUM_RESULTS(TWO, MUL_BY_10, DIV_BY_5)
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
    let main_bytecode = Codegen::generate_main_bytecode(&contract, None).unwrap();

    // Full expected bytecode output (generated from huffc) (placed here as a reference)
    let expected_bytecode = "6002600a0260026005900401";

    // Check the bytecode
    assert_eq!(main_bytecode.to_lowercase(), expected_bytecode.to_lowercase());
}
