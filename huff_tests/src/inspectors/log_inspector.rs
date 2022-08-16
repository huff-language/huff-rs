use bytes::Bytes;
use ethers::{
    prelude::{Address, H256},
    utils::hex,
};
use revm::{CallInputs, CreateInputs, Database, EVMData, Gas, Inspector, Return};
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct LogInspector {
    pub logs: Vec<(u32, String)>,
}

impl<DB> Inspector<DB> for LogInspector
where
    DB: Database,
{
    fn log(&mut self, _: &mut EVMData<'_, DB>, _: &Address, _: &[H256], _: &Bytes) {
        unimplemented!()
    }

    fn call(
        &mut self,
        _: &mut EVMData<'_, DB>,
        call: &mut CallInputs,
        _: bool,
    ) -> (Return, Gas, Bytes) {
        (Return::Continue, Gas::new(call.gas_limit), Bytes::new())
    }

    fn call_end(
        &mut self,
        _: &mut EVMData<'_, DB>,
        call: &CallInputs,
        gas: Gas,
        status: Return,
        retdata: Bytes,
        _: bool,
    ) -> (Return, Gas, Bytes) {
        if call.contract == Address::from_str("00000000000000000000000000000000bEefbabe").unwrap() &&
            call.input.len() == 64
        {
            // In Huffmate, the LOG macro sends 64 bytes of calldata, with the first 32 bytes
            // being the current PC and the second 32 bytes being the value to log.
            /*
            #define macro LOG() = takes (1) {
                // Input stack: [log_item]
                pc             // [pc, log_item]
                0x00 mstore    // [log_item]
                0x20 mstore    // []
                0x00 dup1      // [0x00, 0x00]
                0x40 dup2      // [0x00, 0x40, 0x00, 0x00]
                0x00000000000000000000000000000000bEefbabe
                gas            // [gas, beef_babe, 0x00, 0x40, 0x00, 0x00]
                staticcall pop // []
            }
             */
            let pc = hex::encode(&call.input[0..32]);
            let log_item = hex::encode(&call.input[32..64]);
            self.logs.push((u32::from_str(pc.as_str()).unwrap_or(0), log_item));
        }

        (status, gas, retdata)
    }

    fn create(
        &mut self,
        _: &mut EVMData<'_, DB>,
        call: &mut CreateInputs,
    ) -> (Return, Option<Address>, Gas, Bytes) {
        (Return::Continue, None, Gas::new(call.gas_limit), Bytes::new())
    }

    fn create_end(
        &mut self,
        _: &mut EVMData<'_, DB>,
        _: &CreateInputs,
        status: Return,
        address: Option<Address>,
        gas: Gas,
        retdata: Bytes,
    ) -> (Return, Option<Address>, Gas, Bytes) {
        (status, address, gas, retdata)
    }
}
