use std::collections::BTreeMap;

use huff_lexer::*;
use huff_parser::*;
use huff_utils::prelude::*;

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
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    let abi: Abi = contract.into();
    assert_eq!(
        abi.events,
        BTreeMap::from([(
            "UintEvents".to_string(),
            huff_utils::abi::Event {
                name: "UintEvents".to_string(),
                inputs: vec![
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Uint(256),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Uint(8),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Uint(16),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Uint(32),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Uint(64),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Uint(128),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Uint(256),
                        indexed: false
                    }
                ],
                anonymous: false
            }
        )])
    );
}

#[test]
fn test_abi_int_events() {
    let source: &str = r#"
        #define event IntEvents(int,int8,int16,int32,int64,int128,int256)
        #define macro CONSTRUCTOR() = takes(0) returns (0) {}
        #define macro MAIN() = takes(0) returns (0) {
            0x00 calldataload 0xe0 shr
        }
    "#;

    // Lex + Parse
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    let abi: Abi = contract.into();
    assert_eq!(
        abi.events,
        BTreeMap::from([(
            "IntEvents".to_string(),
            huff_utils::abi::Event {
                name: "IntEvents".to_string(),
                inputs: vec![
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Int(256),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Int(8),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Int(16),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Int(32),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Int(64),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Int(128),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Int(256),
                        indexed: false
                    }
                ],
                anonymous: false
            }
        )])
    );
}

#[test]
fn test_abi_simple_events() {
    let source: &str = r#"
        #define event SimpleEvent(address,address,uint256,string,uint)
        #define macro CONSTRUCTOR() = takes(0) returns (0) {}
        #define macro MAIN() = takes(0) returns (0) {
            0x00 calldataload 0xe0 shr
        }
    "#;

    // Lex + Parse
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    let abi: Abi = contract.into();
    assert_eq!(
        abi.events,
        BTreeMap::from([(
            "SimpleEvent".to_string(),
            huff_utils::abi::Event {
                name: "SimpleEvent".to_string(),
                inputs: vec![
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Address,
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Address,
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Uint(256),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::String,
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Uint(256),
                        indexed: false
                    }
                ],
                anonymous: false
            }
        )])
    );
}

#[ignore]
#[test]
fn test_abi_tuple_array_events() {
    let source: &str = r#"
        #define event TupleArrays(address[],(address,uint256),(uint256,bool)[])
        #define macro CONSTRUCTOR() = takes(0) returns (0) {}
        #define macro MAIN() = takes(0) returns (0) {
            0x00 calldataload 0xe0 shr
        }
    "#;

    // Lex + Parse
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    let abi: Abi = contract.into();
    assert_eq!(
        abi.events,
        BTreeMap::from([(
            "SimpleEvent".to_string(),
            huff_utils::abi::Event {
                name: "SimpleEvent".to_string(),
                inputs: vec![
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Address,
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Address,
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Uint(256),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::String,
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Uint(256),
                        indexed: false
                    }
                ],
                anonymous: false
            }
        )])
    );
}

#[ignore]
#[test]
fn test_abi_nested_tuple_array_events() {
    let source: &str = r#"
        #define event NestedTupleArrays((address,(bool,bytes))[][],bytes,bytes32,string)
        #define macro CONSTRUCTOR() = takes(0) returns (0) {}
        #define macro MAIN() = takes(0) returns (0) {
            0x00 calldataload 0xe0 shr
        }
    "#;

    // Lex + Parse
    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let mut contract = parser.parse().unwrap();
    contract.derive_storage_pointers();

    let abi: Abi = contract.into();
    assert_eq!(
        abi.events,
        BTreeMap::from([(
            "SimpleEvent".to_string(),
            huff_utils::abi::Event {
                name: "SimpleEvent".to_string(),
                inputs: vec![
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Address,
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Address,
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Uint(256),
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::String,
                        indexed: false
                    },
                    EventParam {
                        name: "".to_string(),
                        kind: FunctionParamType::Uint(256),
                        indexed: false
                    }
                ],
                anonymous: false
            }
        )])
    );
}
