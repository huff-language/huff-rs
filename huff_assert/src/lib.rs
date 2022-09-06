use crate::runner::StackRunner;
use ethers::types::{Address, U256};
use huff_codegen::Codegen;
use huff_utils::prelude::Contract;

pub mod errors;
pub mod runner;
pub mod stack;
pub mod utils;

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
            let mut runner = StackRunner::default();

            let name = m.name.to_owned();

            let res = Codegen::macro_to_bytecode(
                m.to_owned(),
                self.ast,
                &mut vec![m.to_owned()],
                0,
                &mut Vec::default(),
            )
            .unwrap();

            let bytecode = Codegen::gen_table_bytecode(res).unwrap();

            let address = runner.deploy_code(bytecode).unwrap();

            // Set environment flags passed through the test decorator
            let mut data = String::default();
            let mut value = U256::zero();

            // Call the deployed test
            let res = runner.call(name, Address::zero(), address, value, data).unwrap();

            dbg!(&res);
        })
    }
}
