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

            let bytecode_res = Codegen::macro_to_bytecode(
                m.to_owned(),
                self.ast,
                &mut vec![m.to_owned()],
                0,
                &mut Vec::default(),
            )
            .unwrap();

            // dbg!(&bytecode_res.bytes);

            let bytecode = Codegen::gen_table_bytecode(bytecode_res.clone()).unwrap();

            let (address, offset) = runner.deploy_code(bytecode).unwrap();

            dbg!(&offset);

            // Set environment flags passed through the test decorator
            let mut data = String::default();
            let mut value = U256::zero();

            // Call the deployed test
            let res = runner
                .call(
                    name,
                    Address::zero(),
                    address,
                    value,
                    data,
                    bytecode_res,
                    /*offset as usize*/ 0,
                )
                .unwrap();

            // dbg!(&res);
        })
    }
}
