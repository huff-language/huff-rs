use crate::runner::StackRunner;
use ethers::types::U256;

use crate::{errors::AssertResult, utils::inspect};
use huff_tests::prelude::TestStatus;
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
            tracing::debug!(target: "assert", "parsing {}", m.name);

            let res = inspect(&self.ast, m, String::default(), U256::zero(), None);

            if res.status == TestStatus::Revert {
                println!("Macro {} reverted", res.name);
            } else {
                if !res.errors.is_empty() {
                    println!("Stack assertion failed at macro {}", res.name);
                    for err in &res.errors {
                        println!("{}", err);
                    }
                }
            }
        })
    }
}
