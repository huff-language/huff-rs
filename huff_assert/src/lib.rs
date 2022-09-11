use crate::runner::StackRunner;
use ethers::types::{Address, U256};

use huff_codegen::Codegen;
use huff_utils::prelude::Contract;

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

            let name = m.name.to_owned();

            let bytecode_res = Codegen::macro_to_bytecode(
                m.to_owned(),
                self.ast,
                &mut vec![m.to_owned()],
                0,
                &mut Vec::default(),
            )
            .unwrap();

            let code = Codegen::gen_table_bytecode(bytecode_res.clone()).unwrap();

            let mut bytecode = if m.takes > 0 {
                // push dummy data to stack
                (0..m.takes).into_iter().map(|_| "6000").collect::<String>()
            } else {
                String::default()
            };
            let offset = bytecode.len() / 2;

            bytecode.push_str(code.as_str());

            let address = runner.deploy_code(bytecode).unwrap();

            let data = String::default();
            let value = U256::zero();

            // Call the deployed test
            let res =
                runner.call(name, Address::zero(), address, value, data, bytecode_res, offset);

            if !res.errors.is_empty() {
                println!("Stack assertion failed at macro {}", res.name);
                for err in res.errors {
                    println!("{:#}", err);
                }
            }
        })
    }
}
