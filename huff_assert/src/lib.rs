use crate::runner::StackRunner;
use ethers::abi::AbiEncode;
use ethers::types::{Address, U256};
use ethers::utils::hex;
use revm::OpCode;

use crate::hex::FromHex;
use huff_codegen::Codegen;
use huff_tests::prelude::TestStatus;
use huff_utils::evm::Opcode::Jump;
use huff_utils::prelude::Contract;
use huff_utils::token::TokenKind::Opcode;

pub mod errors;
pub mod runner;
pub mod stack;

pub struct HuffAssert<'a> {
    ast: &'a Contract,
}

/// Check assertions for a macro considering "takes" and using revm interpreter
impl<'a> HuffAssert<'a> {
    pub fn new(ast: &'a Contract) -> Self {
        HuffAssert { ast }
    }

    pub fn execute(&self) {
        self.ast.macros.iter().for_each(|m| {
            tracing::debug!(target: "assert", "parsing {}", m.name);

            let mut runner = StackRunner::default();

            let bytecode_res = Codegen::macro_to_bytecode(
                m.to_owned(),
                self.ast,
                &mut vec![m.to_owned()],
                0,
                &mut Vec::default(),
            )
            .unwrap();

            let mut code = Codegen::gen_table_bytecode(bytecode_res.clone()).unwrap();

            let address = runner.deploy_code(code).unwrap();

            let data = String::default();
            let value = U256::zero();

            // Call the deployed test
            let res = runner.call(
                m,
                Address::zero(),
                address,
                value,
                data,
                bytecode_res,
                /*offset*/ 0,
            );

            if res.status == TestStatus::Revert {
                println!("Macro {} reverted", res.name);
            } else {
                if !res.errors.is_empty() {
                    println!("Stack assertion failed at macro {}", res.name);
                    for err in res.errors {
                        println!("{:#}", err);
                    }
                }
            }
        })
    }
}
