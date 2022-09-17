use ethers::{prelude::Address, types::U256, utils::hex};

use crate::errors::AssertResult;
use bytes::Bytes;
use huff_tests::errors::RunnerError;

use crate::stack::StackInspector;

use huff_tests::prelude::TestStatus;
use huff_utils::{
    ast::MacroDefinition,
    prelude::{pad_n_bytes, BytecodeRes},
};
use revm::{
    return_ok, AccountInfo, BlockEnv, CfgEnv, CreateScheme, Database, Env, InMemoryDB, Return,
    SpecId, TransactOut, TransactTo, TxEnv, EVM,
};

/// The test runner allows execution of test macros within an in-memory REVM
/// instance.
pub struct StackRunner {
    pub database: InMemoryDB,
    pub env: Env,
}

impl Default for StackRunner {
    fn default() -> Self {
        Self { database: InMemoryDB::default(), env: Env::default() }
    }
}

impl StackRunner {
    /// Get a mutable reference to the database.
    pub fn db_mut(&mut self) -> &mut InMemoryDB {
        &mut self.database
    }

    /// Set the balance of an account.
    pub fn set_balance(&mut self, address: Address, amount: U256) -> &mut Self {
        let db = self.db_mut();

        let account = if let Some(mut account) = db.basic(address).unwrap() {
            account.balance = amount;
            account
        } else {
            AccountInfo { balance: amount, nonce: 0, code_hash: Default::default(), code: None }
        };

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

        // Build maps that will be used later

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
        let res = evm.transact_commit();

        // Check if deployment was successful
        let address = match res.exit_reason {
            return_ok!() => {
                if let TransactOut::Create(_, Some(addr)) = res.out {
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
        m: &MacroDefinition,
        caller: Address,
        address: Address,
        value: U256,
        data: String,
        bytecode_res: BytecodeRes,
        stack: Option<Vec<U256>>,
    ) -> AssertResult {
        let mut evm = EVM::new();

        self.set_balance(caller, U256::MAX);
        evm.env = self.build_env(
            caller,
            TransactTo::Call(address),
            hex::decode(data).expect("Invalid calldata").into(),
            value,
        );
        evm.database(self.db_mut());

        let mut inspector =
            StackInspector::new(bytecode_res.stacks, bytecode_res.last, m.clone(), stack);

        // Send our CALL transaction
        let res = evm.inspect_commit(&mut inspector);

        let status = match res.exit_reason {
            return_ok!() => TestStatus::Success,
            _ => TestStatus::Revert,
        };

        // Return our assert result
        AssertResult { name: m.name.clone(), status, errors: inspector.errors }
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
