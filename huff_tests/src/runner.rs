use crate::{errors::RunnerError, inspector::SimpleInspector};
use bytes::Bytes;
use comfy_table::{Cell, Color};
use ethers::{prelude::Address, types::U256, utils::hex};
use huff_codegen::Codegen;
use huff_utils::{
    ast::{DecoratorFlag, MacroDefinition},
    prelude::{pad_n_bytes, Contract},
};
use revm::{
    return_ok, return_revert, BlockEnv, CfgEnv, CreateScheme, Database, Env, InMemoryDB, Return,
    SpecId, TransactOut, TransactTo, TxEnv, EVM,
};
use yansi::Paint;

/// A test result
#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub return_data: Option<String>,
    pub gas: u64,
    pub status: TestStatus,
}

/// A test status
#[derive(Debug, Clone)]
pub enum TestStatus {
    Success,
    Revert,
}

impl From<TestStatus> for String {
    fn from(status: TestStatus) -> Self {
        match status {
            TestStatus::Success => Paint::green("PASS").to_string(),
            TestStatus::Revert => Paint::red("FAIL").to_string(),
        }
    }
}

impl From<TestStatus> for Cell {
    fn from(status: TestStatus) -> Self {
        match status {
            TestStatus::Success => Cell::new("PASS").fg(Color::Green),
            TestStatus::Revert => Cell::new("FAIL").fg(Color::Red),
        }
    }
}

/// A Test Runner
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

    /// Deploy arbitrary bytecode to our revm instance and return the contract address.
    pub fn deploy_code(&mut self, code: String) -> Result<Address, RunnerError> {
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
        // TODO: Allow the developer to modify the caller, value, and calldata.
        //       Defaults hardcoded for testing / development.
        evm.env = self.build_env(
            Address::zero(),
            TransactTo::Create(CreateScheme::Create),
            hex::decode(bootstrap).expect("Invalid hex").into(),
            U256::zero(),
        );
        evm.database(self.db_mut());

        let (status, out, _, _) = evm.inspect_commit(&mut SimpleInspector::default());

        let address = match status {
            return_ok!() => {
                if let TransactOut::Create(_, Some(addr)) = out {
                    addr
                } else {
                    return Err(RunnerError("Expected contract creation"))
                }
            }
            _ => return Err(RunnerError("Test deployment failed")),
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
        data: String, // TODO: Custom calldata type
    ) -> Result<TestResult, RunnerError> {
        let mut evm = EVM::new();
        self.set_balance(caller, U256::MAX);
        evm.env = self.build_env(
            caller,
            TransactTo::Call(address),
            hex::decode(data).expect("Invalid calldata").into(),
            value,
        );
        evm.database(self.db_mut());

        let (status, out, gas, _) = evm.inspect_commit(&mut SimpleInspector::default());

        let return_data = match status {
            return_ok!() | return_revert!() => {
                if let TransactOut::Call(b) = out {
                    if b.is_empty() {
                        None
                    } else {
                        Some(hex::encode(b))
                    }
                } else {
                    return Err(RunnerError("Expected call"))
                }
            }
            _ => return Err(RunnerError("Expected contract address")),
        };

        Ok(TestResult {
            name,
            return_data,
            gas: gas - 21000,
            status: match status {
                return_ok!() => TestStatus::Success,
                _ => TestStatus::Revert,
            },
        })
    }

    /// Compile a test macro and run it in the revm instance.
    pub fn run_test(
        &mut self,
        m: &MacroDefinition,
        contract: &Contract,
    ) -> Result<TestResult, RunnerError> {
        let name = m.name.clone();

        if let Ok(res) = Codegen::macro_to_bytecode(
            m.clone(),
            contract,
            &mut vec![m.clone()],
            0,
            &mut Vec::default(),
        ) {
            if let Ok(bytecode) = Codegen::gen_table_bytecode(res) {
                let address = self.deploy_code(bytecode)?;

                let mut data = String::default();
                let mut value = U256::zero();
                if let Some(decorator) = &m.decorator {
                    for flag in &decorator.flags {
                        match flag {
                            DecoratorFlag::Calldata(s) => {
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

                let res = self.call(name, Address::zero(), address, value, data)?;

                return Ok(res)
            }
        }
        // TODO: Print error from codegen
        Err(RunnerError("Failed to generate bytecode"))
    }

    /// Build an EVM transaction environment.
    fn build_env(&self, caller: Address, to: TransactTo, data: Bytes, value: U256) -> Env {
        // TODO: Allow the developer to change these values
        //       Defaults hardcoded for testing / development.
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
