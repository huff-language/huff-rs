use crate::prelude::{cheats_inspector::CheatsInspector, RunnerError, TestResult, TestStatus};
use bytes::Bytes;
use ethers::{prelude::Address, types::U256, utils::hex};
use huff_codegen::Codegen;
use huff_utils::{
    ast::{DecoratorFlag, MacroDefinition},
    prelude::{pad_n_bytes, CompilerError, Contract},
};
use revm::{
    return_ok, return_revert, BlockEnv, CfgEnv, CreateScheme, Database, Env, InMemoryDB, Return,
    SpecId, TransactOut, TransactTo, TxEnv, EVM,
};

/// The test runner allows execution of test macros within an in-memory REVM
/// instance.
pub struct TestRunner {
    pub database: InMemoryDB,
    pub env: Env,
}

impl Default for TestRunner {
    fn default() -> Self {
        Self { database: InMemoryDB::default(), env: Env::default() }
    }
}

impl TestRunner {
    /// Get a mutable reference to the database.
    pub fn db_mut(&mut self) -> &mut InMemoryDB {
        &mut self.database
    }

    /// Set the balance of an account.
    pub fn set_balance(&mut self, address: Address, amount: U256) -> &mut Self {
        let db = self.db_mut();

        let mut account = db.basic(address);
        account.balance = amount;
        db.insert_account_info(address, account);

        self
    }

    /// Deploy arbitrary bytecode to our REVM instance and return the contract address.
    pub fn deploy_code(&mut self, code: String) -> Result<Address, RunnerError> {
        // Wrap code in a bootstrap constructor
        let contract_length = code.len() / 2;
        let constructor_length = 0;
        let mut bootstrap_code_size = 9;
        let contract_size = if contract_length < 256 {
            format!("60{}", pad_n_bytes(format!("{:x}", contract_length).as_str(), 1))
        } else {
            bootstrap_code_size += 1;

            format!("61{}", pad_n_bytes(format!("{:x}", contract_length).as_str(), 2))
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
        let bootstrap = format!("{}80{}3d393df3{}", contract_size, contract_code_offset, code);

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
        let (status, out, _, _) = evm.transact_commit();

        // Check if deployment was successful
        let address = match status {
            return_ok!() => {
                if let TransactOut::Create(_, Some(addr)) = out {
                    addr
                } else {
                    return Err(RunnerError(String::from("Test deployment failed")))
                }
            }
            _ => return Err(RunnerError(String::from("Test deployment failed"))),
        };
        Ok(address)
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
        evm.env = self.build_env(
            caller,
            TransactTo::Call(address),
            hex::decode(data).expect("Invalid calldata").into(),
            value,
        );
        evm.database(self.db_mut());

        // Send our CALL transaction
        let (status, out, gas, _) = evm.inspect_commit(&mut inspector);

        // Check if the transaction was successful
        let return_data = match status {
            return_ok!() | return_revert!() => {
                if let TransactOut::Call(b) = out {
                    if b.is_empty() {
                        None
                    } else {
                        Some(hex::encode(b))
                    }
                } else {
                    return Err(RunnerError(String::from("Unexpected transaction kind")))
                }
            }
            _ => return Err(RunnerError(String::from("Unexpected transaction status"))),
        };

        // Return our test result
        // NOTE: We subtract 21000 gas from the gas result to account for the
        // base cost of the CALL.
        Ok(TestResult {
            name,
            return_data,
            gas: gas - 21000,
            status: match status {
                return_ok!() => TestStatus::Success,
                _ => TestStatus::Revert,
            },
            logs: inspector.logs,
        })
    }

    /// Compile a test macro and run it in an in-memory REVM instance.
    pub fn run_test(
        &mut self,
        m: &MacroDefinition,
        contract: &Contract,
    ) -> Result<TestResult, RunnerError> {
        let name = m.name.to_owned();

        // Compile the passed test macro
        match Codegen::macro_to_bytecode(
            m.to_owned(),
            contract,
            &mut vec![m.to_owned()],
            0,
            &mut Vec::default(),
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
        Env {
            cfg: CfgEnv { chain_id: 1.into(), spec_id: SpecId::LATEST, ..Default::default() },
            block: BlockEnv { basefee: 0.into(), gas_limit: U256::MAX, ..Default::default() },
            tx: TxEnv {
                chain_id: 1.into(),
                caller,
                transact_to: to,
                data,
                value,
                ..Default::default()
            },
        }
    }
}
