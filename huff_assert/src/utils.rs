use crate::{AssertResult, StackRunner};
use ethers::types::{Address, U256};
use huff_codegen::Codegen;
use huff_utils::ast::{Contract, MacroDefinition};
use std::fmt::Debug;

pub fn inspect(
    ast: &Contract,
    m: &MacroDefinition,
    data: String,
    value: U256,
    stack: Option<Vec<U256>>,
) -> AssertResult {
    let mut runner = StackRunner::default();

    let bytecode_res = Codegen::macro_to_bytecode(
        m.to_owned(),
        ast,
        &mut vec![m.to_owned()],
        0,
        &mut Vec::default(),
    )
    .unwrap();

    let code = Codegen::gen_table_bytecode(bytecode_res.clone()).unwrap();

    let address = runner.deploy_code(code).unwrap();

    runner.call(m, Address::zero(), address, value, data, bytecode_res, stack)
}

pub fn format_arr<T: Debug>(arr: Vec<T>) -> String {
    let s = format!("`{:?}`", arr).replace('"', "");
    format!("{}", s)
}
