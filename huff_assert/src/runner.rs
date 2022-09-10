use ethers::{prelude::Address, types::U256, utils::hex};

use std::collections::BTreeMap;

use crate::errors::AssertResult;
use bytes::Bytes;
use huff_tests::errors::RunnerError;

use crate::stack::StackInspector;
use huff_utils::prelude::pad_n_bytes;
use huff_utils::prelude::{BytecodeRes, Bytes as HuffBytes};
use revm::{
    return_ok, BlockEnv, CfgEnv, CreateScheme, Database, Env, InMemoryDB, Return, SpecId,
    TransactOut, TransactTo, TxEnv, EVM,
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
        let (status, out, _, _) = evm.transact_commit();

        // Check if deployment was successful
        let address = match status {
            return_ok!() => {
                if let TransactOut::Create(_, Some(addr)) = out {
                    addr
                } else {
                    return Err(RunnerError(String::from("Test deployment failed")));
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
        bytecode_res: BytecodeRes,
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

        let mut pc_to_assert: BTreeMap<usize, HuffBytes> = BTreeMap::new();
        bytecode_res.bytes.into_iter().filter(|(_, b)| b.0.starts_with("stack: ")).for_each(
            |(c, b)| {
                pc_to_assert.insert(c, b);
            },
        );

        let mut inspector = StackInspector::new(pc_to_assert);

        // Send our CALL transaction
        /*let (status, ..) =*/
        evm.inspect_commit(&mut inspector);

        // Should we enforce the tx to pass or only check for stack ?
        /*match status {
            return_ok!() | return_revert!() | Return::FatalNotSupported => {
                if let TransactOut::Call(b) = out {
                    if b.is_empty() {
                        None
                    } else {
                        Some(hex::encode(b))
                    }
                } else {
                    dbg!(&out);
                    return Err(RunnerError(String::from("Unexpected transaction kind")));
                }
            }
            _ => return Err(RunnerError(String::from("Unexpected transaction status"))),
        };*/

        // Return our assert result
        AssertResult { name, errors: inspector.errors }
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
