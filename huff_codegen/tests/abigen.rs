use std::collections::BTreeMap;

use huff_codegen::Codegen;
use huff_utils::{ast, prelude::*};

#[test]
fn constructs_valid_abi() {
    let constructor = ast::Function {
        name: "CONSTRUCTOR",
        signature: [0u8, 0u8, 0u8, 0u8],
        inputs: vec![],
        fn_type: FunctionType::NonPayable,
        outputs: vec![],
    };
    let contract = Contract {
        macros: vec![],
        invocations: vec![],
        imports: vec![],
        constants: vec![],
        functions: vec![constructor.clone()],
        events: vec![],
        tables: vec![],
    };

    // Generate the abi from the contract
    let mut cg = Codegen::new(true);
    let abi = cg.abigen(contract).unwrap();
    println!("Abi: {:?}", abi);
    assert_eq!(
        abi,
        Abi {
            constructor: Some(Constructor { inputs: vec![] }),
            functions: BTreeMap::new(),
            events: BTreeMap::new(),
            receive: false,
            fallback: false
        }
    );
}

#[test]
#[should_panic]
fn missing_constructor_fails() {
    let _constructor = ast::Function {
        name: "CONSTRUCTOR",
        signature: [0u8, 0u8, 0u8, 0u8],
        inputs: vec![],
        fn_type: FunctionType::NonPayable,
        outputs: vec![],
    };
    let contract = Contract {
        macros: vec![],
        invocations: vec![],
        imports: vec![],
        constants: vec![],
        functions: vec![],
        events: vec![],
        tables: vec![],
    };

    // Generate the abi from the contract
    // This should fail since there's no constructor
    let mut cg = Codegen::new(true);
    let abi = cg.abigen(contract);
    assert!(abi.is_ok())
}
