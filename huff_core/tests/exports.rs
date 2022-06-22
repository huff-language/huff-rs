use std::collections::BTreeMap;

use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

// #define event UintEvents(uint,uint8,uint16,uint32,uint64,uint128,uint256)
// #define event IntEvents(int,int8,int16,int32,int64,int128,int256)
// #define event SimpleEvent(address,address,uint256,string,uint)
// #define event TupleArrays(address[],(address,uint256),(uint256,bool)[])
// #define event NestedTupleArrays((address,(bool,bytes))[][],bytes,bytes32,string)

#[test]
fn test_abi_uint_events() {
  let source: &str = r#"
    #define event UintEvents(uint,uint8,uint16,uint32,uint64,uint128,uint256)
    #define macro CONSTRUCTOR() = takes(0) returns (0) {}
    #define macro MAIN() = takes(0) returns (0) {
      0x00 calldataload 0xe0 shr
    }
  "#;

  // Lex + Parse
  let flattened_source = FullFileSource { source, file: None, spans: vec![] };
  let lexer = Lexer::new(flattened_source);
  let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
  let mut parser = Parser::new(tokens, None);
  let mut contract = parser.parse().unwrap();
  contract.derive_storage_pointers();

  // let main_bytecode = Codegen::generate_main_bytecode(&contract).unwrap();
  // let constructor_bytecode = Codegen::generate_constructor_bytecode(&contract).unwrap();

  // let mut cg = Codegen::new();
  // let churn_res = cg.churn(std::sync::Arc::new(FileSource::default()), vec![], &main_bytecode, &constructor_bytecode).unwrap();

  let abi: Abi = contract.into();
  assert_eq!(abi.events, BTreeMap::from(vec![
    ("UintEvents".to_string(), Event {
      name: "UintEvents".to_string(),
      inputs: vec![
        EventParam {
          name: "".to_string(),
          kind: FunctionParamType::Uint(8),
          indexed: false
        }
      ],
      anonymous: false
    })
  ]));
}


#[test]
fn test_abi_int_events() {
  let source: &str = r#"
    #define event UintEvents(uint,uint8,uint16,uint32,uint64,uint128,uint256)
    #define macro CONSTRUCTOR() = takes(0) returns (0) {}
    #define macro MAIN() = takes(0) returns (0) {
      0x00 calldataload 0xe0 shr
    }
  "#;

  // Lex + Parse
  let flattened_source = FullFileSource { source, file: None, spans: vec![] };
  let lexer = Lexer::new(flattened_source);
  let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
  let mut parser = Parser::new(tokens, None);
  let mut contract = parser.parse().unwrap();
  contract.derive_storage_pointers();

  // let main_bytecode = Codegen::generate_main_bytecode(&contract).unwrap();
  // let constructor_bytecode = Codegen::generate_constructor_bytecode(&contract).unwrap();

  // let mut cg = Codegen::new();
  // let churn_res = cg.churn(std::sync::Arc::new(FileSource::default()), vec![], &main_bytecode, &constructor_bytecode).unwrap();

  let abi: Abi = contract.into();
  assert_eq!(abi.events, BTreeMap::from(vec![
    ("UintEvents".to_string(), Event {
      name: "UintEvents".to_string(),
      inputs: vec![
        EventParam {
          name: "".to_string(),
          kind: FunctionParamType::Uint(8),
          indexed: false
        }
      ],
      anonymous: false
    })
  ]));
}