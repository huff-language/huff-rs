use crate::{AssertResult, StackRunner};
use ethers::types::{Address, U256};
use huff_codegen::Codegen;
use huff_utils::ast::{Contract, MacroDefinition};
use revm::Return;
use std::fmt::{Debug, Display, Formatter};

pub fn inspect(
    ast: &Contract,
    m: &MacroDefinition,
    data: String,
    value: U256,
    stack: Vec<U256>,
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
    format!("`{:?}`", arr).replace('"', "")
}

pub struct RevmReturn(pub Return);

impl Display for RevmReturn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let m: &str = match self.0 {
            Return::Continue => "Continue",
            Return::Stop => "Stop",
            Return::Return => "Return",
            Return::SelfDestruct => "SelfDestruct",
            Return::Revert => "Revert",
            Return::CallTooDeep => "CallTooDeep",
            Return::OutOfFund => "OutOfFund",
            Return::OutOfGas => "OutOfGas",
            Return::OpcodeNotFound => "OpcodeNotFound",
            Return::CallNotAllowedInsideStatic => "CallNotAllowedInsideStatic",
            Return::InvalidOpcode => "InvalidOpcode",
            Return::InvalidJump => "InvalidJump",
            Return::InvalidMemoryRange => "InvalidMemoryRange",
            Return::NotActivated => "NotActivated",
            Return::StackUnderflow => "StackUnderflow",
            Return::StackOverflow => "StackOverflow",
            Return::OutOfOffset => "OutOfOffset",
            Return::FatalExternalError => "FatalExternalError",
            Return::GasMaxFeeGreaterThanPriorityFee => "GasMaxFeeGreaterThanPriorityFee",
            Return::GasPriceLessThenBasefee => "GasPriceLessThenBasefee",
            Return::CallerGasLimitMoreThenBlock => "CallerGasLimitMoreThenBlock",
            Return::RejectCallerWithCode => "RejectCallerWithCode",
            Return::LackOfFundForGasLimit => "LackOfFundForGasLimit",
            Return::CreateCollision => "CreateCollision",
            Return::OverflowPayment => "OverflowPayment",
            Return::PrecompileError => "PrecompileError",
            Return::NonceOverflow => "NonceOverflow",
            Return::CreateContractLimit => "CreateContractLimit",
            Return::CreateContractWithEF => "CreateContractWithEF",
        };

        write!(f, "{}", m)
    }
}
