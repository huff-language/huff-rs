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

// #[test]
// #[should_panic]
// fn missing_constructor_fails() {
//     let c = "#define constant LITERAL =
// 0x8C5BE1E5EBEC7D5BD14F71427D1E84F3DD0314C0F7B2291E5B200AC8C7C3B925";

//     let lexer = Lexer::new(c);
//     let tokens = lexer.into_iter().map(|x| x.unwrap()).collect::<Vec<Token>>();
//     let mut parser = Parser::new(tokens);
//     let contract = parser.parse().unwrap();
//     assert_eq!(parser.current_token.kind, TokenKind::Eof);

//     // Create const val
//     let mut arr: [u8; 32] = Default::default();
//     let mut buf =
//         BytesMut::from("8C5BE1E5EBEC7D5BD14F71427D1E84F3DD0314C0F7B2291E5B200AC8C7C3B925");
//     buf.resize(32, 0);
//     arr.copy_from_slice(buf.as_ref());

//     // Check Literal
//     let fsp_constant = contract.constants[0].clone();
//     assert_eq!(fsp_constant, ConstantDefinition { name: "LITERAL", value: ConstVal::Literal(arr)
// }); }
