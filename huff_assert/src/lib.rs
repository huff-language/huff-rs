use crate::runner::StackRunner;
use ethers::{
    types::{Address, U256},
    utils::hex,
};

use crate::errors::AssertResult;
use huff_codegen::Codegen;
use huff_tests::prelude::TestStatus;
use huff_utils::{ast::MacroDefinition, prelude::Contract};

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

    // This should return a Result
    pub fn inspect(&self, m: &MacroDefinition, data: String, value: U256) -> AssertResult {
        let mut runner = StackRunner::default();

        let bytecode_res = Codegen::macro_to_bytecode(
            m.to_owned(),
            self.ast,
            &mut vec![m.to_owned()],
            0,
            &mut Vec::default(),
        )
        .unwrap();

        let code = Codegen::gen_table_bytecode(bytecode_res.clone()).unwrap();

        let address = runner.deploy_code(code).unwrap();

        runner.call(m, Address::zero(), address, value, data, bytecode_res)
    }

    pub fn execute(&self) {
        self.ast.macros.iter().for_each(|m| {
            tracing::debug!(target: "assert", "parsing {}", m.name);

            let res = &self.inspect(m, String::default(), U256::zero());

            if res.status == TestStatus::Revert {
                println!("Macro {} reverted", res.name);
            } else {
                if !res.errors.is_empty() {
                    println!("Stack assertion failed at macro {}", res.name);
                    for err in &res.errors {
                        println!("{:#}", err);
                    }
                }
            }
        })
    }
}
