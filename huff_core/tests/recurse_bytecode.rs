use std::collections::BTreeMap;

use huff_codegen::Codegen;
use huff_lexer::Lexer;
use huff_parser::Parser;
use huff_utils::{evm::Opcode, prelude::*};

#[test]
fn recurse_macro_bytecode() {
    let source = r#"
    /* Constructor */

    #define macro CONSTRUCTOR() = takes(0) returns (0) {}

    #define macro TRANSFER() = takes(0) returns(1) {
      0x04 calldataload
      caller
      0x24 calldataload

      /* THIS FUNCTION DOESN'T ACTUALLY TRANSFER */

      0x01 0x00 mstore
      0x20 0x00 return
    }

    #define macro MINT() = takes(0) returns (0) {
        0x04 calldataload   // [to]
        0x00                // [from (0x00), to]
        0x24 calldataload   // [value, from, to]

        // UPDATE TOTAL SUPPLY
        dup1                             // [value, value, from, to]
        [TOTAL_SUPPLY_LOCATION] sload    // [supply,value,value,from,to]
        add                              // [supply+value,value,from,to]
        [TOTAL_SUPPLY_LOCATION] sstore   // [value,from,to]
    }

    #define macro MAIN() = takes(0) returns (0) {
        0x00 calldataload 0xE0 shr
        dup1 0xa9059cbb eq transfer jumpi
        dup1 0x40c10f19 eq mints jumpi

        transfer:
            TRANSFER()
        mints:
            MINT()
    }
    "#;

    // Lex + Parse
    let lexer = Lexer::new(source);
    let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
    let mut parser = Parser::new(tokens, None);
    let ast = parser.parse().unwrap();

    let contract = Contract {
        macros: vec![
            MacroDefinition {
                name: "CONSTRUCTOR".to_string(),
                parameters: vec![],
                statements: vec![],
                takes: 0,
                returns: 0,
            },
            MacroDefinition {
                name: "TRANSFER".to_string(),
                parameters: vec![],
                statements: vec![
                    Statement::Literal(str_to_bytes32("04")),
                    Statement::Opcode(Opcode::Calldataload),
                    Statement::Opcode(Opcode::Caller),
                    Statement::Literal(str_to_bytes32("24")),
                    Statement::Opcode(Opcode::Calldataload),
                    Statement::Literal(str_to_bytes32("01")),
                    Statement::Literal(str_to_bytes32("00")),
                    Statement::Opcode(Opcode::Mstore),
                    Statement::Literal(str_to_bytes32("20")),
                    Statement::Literal(str_to_bytes32("00")),
                    Statement::Opcode(Opcode::Return),
                ],
                takes: 0,
                returns: 1,
            },
            MacroDefinition {
                name: "MINT".to_string(),
                parameters: vec![],
                statements: vec![
                    Statement::Literal(str_to_bytes32("04")),
                    Statement::Opcode(Opcode::Calldataload),
                    Statement::Literal(str_to_bytes32("00")),
                    Statement::Literal(str_to_bytes32("24")),
                    Statement::Opcode(Opcode::Calldataload),
                    Statement::Opcode(Opcode::Dup1),
                    Statement::Constant("TOTAL_SUPPLY_LOCATION".to_string()),
                    Statement::Opcode(Opcode::Sload),
                    Statement::Opcode(Opcode::Add),
                    Statement::Constant("TOTAL_SUPPLY_LOCATION".to_string()),
                    Statement::Opcode(Opcode::Sstore),
                ],
                takes: 0,
                returns: 0,
            },
            MacroDefinition {
                name: "MAIN".to_string(),
                parameters: vec![],
                statements: vec![
                    Statement::Literal(str_to_bytes32("00")),
                    Statement::Opcode(Opcode::Calldataload),
                    Statement::Literal(str_to_bytes32("E0")),
                    Statement::Opcode(Opcode::Shr),
                    Statement::Opcode(Opcode::Dup1),
                    Statement::Literal(str_to_bytes32("a9059cbb")),
                    Statement::Opcode(Opcode::Eq),
                    Statement::LabelCall("transfer".to_string()),
                    Statement::Opcode(Opcode::Jumpi),
                    Statement::Opcode(Opcode::Dup1),
                    Statement::Literal(str_to_bytes32("40c10f19")),
                    Statement::Opcode(Opcode::Eq),
                    Statement::LabelCall("mints".to_string()),
                    Statement::Opcode(Opcode::Jumpi),
                    Statement::Label(Label {
                        name: "transfer".to_string(),
                        inner: vec![Statement::MacroInvocation(MacroInvocation {
                            macro_name: "TRANSFER".to_string(),
                            args: vec![],
                        })],
                    }),
                    Statement::Label(Label {
                        name: "mints".to_string(),
                        inner: vec![Statement::MacroInvocation(MacroInvocation {
                            macro_name: "MINT".to_string(),
                            args: vec![],
                        })],
                    }),
                ],
                takes: 0,
                returns: 0,
            },
        ],
        invocations: vec![],
        imports: vec![],
        constants: vec![],
        functions: vec![],
        events: vec![],
        tables: vec![],
    };

    // Sanity Check The AST
    assert_eq!(contract, ast);

    // Main Macro Definition
    let main_macro =
        contract.macros.iter().filter(|m| m.name == "MAIN").collect::<Vec<&MacroDefinition>>()[0]
            .clone();

    let offset = 0;
    let jump_tables = vec![];
    let bytecode_res: BytecodeRes = Codegen::recurse_bytecode(
        main_macro.clone(),
        Some(ast),
        &mut vec![main_macro],
        offset,
        jump_tables,
    )
    .unwrap();

    // Validate bytecode result
    assert_eq!(
        bytecode_res,
        BytecodeRes {
            bytes: vec![],
            jump_tables: vec![BTreeMap::new()],
            jump_indices: BTreeMap::new(),
            unmatched_jumps: vec![]
        }
    );
}
