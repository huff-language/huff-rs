use crate::runner::StackRunner;
use ethers::types::U256;

use crate::{
    errors::{AssertResult, PrettyError},
    utils::inspect,
};
use huff_tests::prelude::TestStatus;
use huff_utils::prelude::{Contract, MacroInput};
use yansi::Paint;

use crate::utils::RevmReturn;
use revm::{return_ok, Return};
use std::collections::BTreeMap;

pub mod errors;
pub mod runner;
pub mod stack;
pub mod utils;

pub struct HuffAssert<'a, 'b> {
    ast: &'a Contract,
    macros_val: &'b BTreeMap<String, MacroInput>,
}

/// Check assertions for a macro considering "takes" and "returns" and using revm interpreter
impl<'a, 'b> HuffAssert<'a, 'b> {
    pub fn new(ast: &'a Contract, macros_val: &'b BTreeMap<String, MacroInput>) -> Self {
        HuffAssert { ast, macros_val }
    }

    pub fn execute(&self) {
        self.ast.macros.iter().for_each(|m| {
            tracing::debug!(target: "assert", "parsing {}", m.name);

            let name = &m.name;
            let (data, stack) = get_val(self.macros_val, name);

            let mut stack_vec: Vec<U256> = Vec::new();

            if stack == "_" || stack.is_empty() {
                for _ in 0..m.takes {
                    stack_vec.push(U256::zero());
                }
            } else if stack == "x" {
                // do nothing
            } else {
                stack.chars().enumerate().into_iter().for_each(|(i, _)| {
                    let num = i + 1;
                    if num % 32 == 0 && num != 1 {
                        let sub_vec = U256::from_str_radix(&stack[(num - 32)..num], 16).unwrap();
                        stack_vec.push(sub_vec);
                    }
                });
                stack_vec = stack_vec.into_iter().rev().collect::<Vec<U256>>();
            }

            let data = if data == "x" { "" } else { data };

            let res = inspect(self.ast, m, data.to_string(), U256::zero(), stack_vec);

            let status = match res.reason {
                return_ok!() => TestStatus::Success,
                _ => TestStatus::Revert,
            };

            if status == TestStatus::Revert {
                eprintln!(
                    "{}",
                    Paint::red(format!(
                        "Macro {} reverted with {}",
                        res.name,
                        RevmReturn(res.reason)
                    ))
                );
            } else if !res.errors.is_empty() {
                eprintln!(
                    "{}",
                    Paint::red(format!("Stack assertion failed at macro {}", res.name))
                );
                for err in &res.errors {
                    let p_err = PrettyError(err.clone());
                    eprintln!("{}", Paint::red(p_err));
                }
            }
        })
    }
}

fn get_val<'a>(macros_val: &'a BTreeMap<String, MacroInput>, name: &str) -> (&'a str, &'a str) {
    match macros_val.get(name) {
        Some(val) => (&*val.data, &*val.stack),
        None => ("", ""),
    }
}
