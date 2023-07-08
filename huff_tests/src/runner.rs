use crate::prelude::{cheats_inspector::CheatsInspector, RunnerError, TestResult, TestStatus};
use bytes::Bytes;
use ethers_core::{
    types::{Address, U256},
    utils::hex,
};
use huff_codegen::Codegen;
use huff_utils::{
    ast::{DecoratorFlag, MacroDefinition},
    prelude::{pad_n_bytes, CompilerError, Contract, EVMVersion},
};
use revm::{
    db::DbAccount,
    primitives::{
        BlockEnv, CfgEnv, CreateScheme, Env, ExecutionResult, Output, SpecId, TransactTo, TxEnv,
    },
    Database, InMemoryDB, EVM,
};

/// The test runner allows execution of test macros within an in-memory REVM
/// instance.
#[derive(Default, Debug)]
pub struct TestRunner {
    pub database: InMemoryDB,
    pub env: Env,
}

impl TestRunner {
    /// Get a mutable reference to the database.
    pub fn db_mut(&mut self) -> &mut InMemoryDB {
        &mut self.database
    }

    /// Set the balance of an account.
    pub fn set_balance(&mut self, address: Address, amount: U256) -> &mut Self {
        let db = self.db_mut();

        let revm_address = revm::primitives::B160::from_slice(address.as_bytes());
        let mut account = match db.basic(revm_address) {
            Ok(Some(info)) => DbAccount { info, ..Default::default() },
            _ => DbAccount::new_not_existing(),
        };
        account.info.balance = amount.into();
        db.insert_account_info(revm_address, account.info);

        self
    }

    /// Deploy arbitrary bytecode to our REVM instance and return the contract address.
    pub fn deploy_code(&mut self, code: String) -> Result<Address, RunnerError> {
        // Wrap code in a bootstrap constructor
        let contract_length = code.len() / 2;
        let constructor_length = 0;
        let mut bootstrap_code_size = 9;
        let contract_size = if contract_length < 256 {
            format!("60{}", pad_n_bytes(format!("{contract_length:x}").as_str(), 1))
        } else {
            bootstrap_code_size += 1;

            format!("61{}", pad_n_bytes(format!("{contract_length:x}").as_str(), 2))
        };
        let contract_code_offset = if (bootstrap_code_size + constructor_length) < 256 {
            format!(
                "60{}",
                pad_n_bytes(format!("{:x}", bootstrap_code_size + constructor_length).as_str(), 1)
            )
        } else {
            bootstrap_code_size += 1;

            format!(
                "61{}",
                pad_n_bytes(format!("{:x}", bootstrap_code_size + constructor_length).as_str(), 2)
            )
        };
        let bootstrap = format!("{contract_size}80{contract_code_offset}3d393df3{code}");

        let mut evm = EVM::new();
        self.set_balance(Address::zero(), U256::MAX);
        evm.env = self.build_env(
            Address::zero(),
            TransactTo::Create(CreateScheme::Create),
            // The following should never panic, as any potential compilation error
            // as well as an uneven number of hex nibbles should be caught in the
            // compilation process.
            hex::decode(bootstrap).expect("Invalid hex").into(),
            U256::zero(),
        );
        evm.database(self.db_mut());

        // Send our CREATE transaction
        let er = evm.transact_commit().map_err(RunnerError::from)?;

        // Check if deployment was successful
        let address = match er {
            ExecutionResult::Success { output: Output::Create(_, Some(addr)), .. } => addr,
            _ => return Err(RunnerError(String::from("Test deployment failed"))),
        };
        let ethers_address = ethers_core::types::Address::from_slice(address.as_bytes());
        Ok(ethers_address)
    }

    /// Perform a call to a deployed contract
    pub fn call(
        &mut self,
        name: String,
        caller: Address,
        address: Address,
        value: U256,
        data: String,
    ) -> Result<TestResult, RunnerError> {
        let mut evm = EVM::new();
        let mut inspector = CheatsInspector::default();
        self.set_balance(caller, U256::MAX);
        let revm_address = revm::primitives::B160::from_slice(address.as_bytes());

        evm.env = self.build_env(
            caller,
            TransactTo::Call(revm_address),
            hex::decode(data).expect("Invalid calldata").into(),
            value,
        );
        evm.database(self.db_mut());

        // Send our CALL transaction
        let er = evm.inspect_commit(&mut inspector).map_err(RunnerError::from)?;

        // Extract execution params
        let gas_used = match er {
            ExecutionResult::Success { gas_used, .. } => gas_used,
            ExecutionResult::Revert { gas_used, .. } => gas_used,
            _ => return Err(RunnerError(String::from("Unexpected transaction status"))),
        };
        let status = match er {
            ExecutionResult::Success { .. } => TestStatus::Success,
            _ => TestStatus::Revert,
        };

        // Check if the transaction was successful
        let return_data = match er {
            ExecutionResult::Success { output, .. } => {
                if let Output::Call(b) = output {
                    if b.is_empty() {
                        None
                    } else {
                        Some(hex::encode(b))
                    }
                } else {
                    return Err(RunnerError(String::from("Unexpected transaction kind")))
                }
            }
            ExecutionResult::Revert { output, .. } => {
                if output.is_empty() {
                    None
                } else {
                    Some(hex::encode(output))
                }
            }
            _ => return Err(RunnerError(String::from("Unexpected transaction status"))),
        };

        // Return our test result
        // NOTE: We subtract 21000 gas from the gas result to account for the
        // base cost of the CALL.
        Ok(TestResult { name, return_data, gas: gas_used - 21000, status, logs: inspector.logs })
    }

    /// Compile a test macro and run it in an in-memory REVM instance.
    pub fn run_test(
        &mut self,
        m: &MacroDefinition,
        contract: &Contract,
    ) -> Result<TestResult, RunnerError> {
        // TODO: set to non default
        let evm_version = EVMVersion::default();

        let name = m.name.to_owned();

        // Compile the passed test macro
        match Codegen::macro_to_bytecode(
            &evm_version,
            m,
            contract,
            &mut vec![m],
            0,
            &mut Vec::default(),
            false,
            None,
        ) {
            // Generate table bytecode for compiled test macro
            Ok(res) => match Codegen::gen_table_bytecode(res) {
                Ok(bytecode) => {
                    // Deploy compiled test macro
                    let address = self.deploy_code(bytecode)?;

                    // Set environment flags passed through the test decorator
                    let mut data = String::default();
                    let mut value = U256::zero();
                    if let Some(decorator) = &m.decorator {
                        for flag in &decorator.flags {
                            match flag {
                                DecoratorFlag::Calldata(s) => {
                                    // Strip calldata of 0x prefix, if it is present.
                                    data = if let Some(s) = s.strip_prefix("0x") {
                                        s.to_owned()
                                    } else {
                                        s.to_owned()
                                    };
                                }
                                DecoratorFlag::Value(v) => value = U256::from(v),
                            }
                        }
                    }

                    // Call the deployed test
                    let res = self.call(name, Address::zero(), address, value, data)?;
                    Ok(res)
                }
                Err(e) => Err(CompilerError::CodegenError(e).into()),
            },
            Err(e) => Err(CompilerError::CodegenError(e).into()),
        }
    }

    /// Build an EVM transaction environment.
    fn build_env(&self, caller: Address, to: TransactTo, data: Bytes, value: U256) -> Env {
        let revm_address = revm::primitives::B160::from_slice(caller.as_bytes());
        Env {
            cfg: CfgEnv {
                chain_id: revm::primitives::U256::from(1),
                spec_id: SpecId::LATEST,
                ..Default::default()
            },
            block: BlockEnv {
                basefee: revm::primitives::U256::from(0),
                gas_limit: U256::MAX.into(),
                ..Default::default()
            },
            tx: TxEnv {
                chain_id: 1.into(),
                caller: revm_address,
                transact_to: to,
                data,
                value: value.into(),
                ..Default::default()
            },
        }
    }
}
