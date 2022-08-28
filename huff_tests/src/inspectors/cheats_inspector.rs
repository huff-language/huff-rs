use crate::cheats::{HuffCheatCode, HUFF_CHEATS_MAP};
use bytes::Bytes;
use ethers::{
    prelude::{Address, H256},
    utils::hex,
};
use lazy_static::lazy_static;
use revm::{CallInputs, CreateInputs, Database, EVMData, Gas, Inspector, Return};
use std::str::FromStr;

lazy_static! {
    pub static ref CHEATS_ADDR: Address =
        Address::from_str("00000000000000000000000000000000bEefbabe").unwrap();
}

#[derive(Debug, Default)]
pub struct CheatsInspector {
    pub logs: Vec<(u32, String)>,
}

impl<DB> Inspector<DB> for CheatsInspector
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
        if call.contract == *CHEATS_ADDR && call.input.len() >= 64 {
            // All cheatcodes calls must include the cheatcode key and the current pc in the first
            // 64 bytes of calldata.
            fn bytes_to_u32(b: &[u8]) -> u32 {
                u32::from_str_radix(hex::encode(b).as_str(), 16).unwrap_or(0)
            }
            let cheat_key = bytes_to_u32(&call.input[0..32]);
            let pc = bytes_to_u32(&call.input[32..64]);

            if let Some(HuffCheatCode::Log) = HUFF_CHEATS_MAP.get(&cheat_key) {
                // In Huffmate, the LOG macro sends 96 bytes of calldata to our cheatcode
                // address, laid out as follows:
                // ╔════════╦═══════════════╗
                // ║ Offset ║     Value     ║
                // ╠════════╬═══════════════╣
                // ║ 0x00   ║ cheat_key     ║
                // ║ 0x20   ║ pc            ║
                // ║ 0x40   ║ log_item      ║
                // ╚════════╩═══════════════╝
                //
                // #define macro LOG() = takes (1) {
                //     // Input stack:   [log_item]
                //     pc             // [pc, log_item]
                //     0x01           // [log_cheatcode, pc, log_item]
                //     0x00 mstore    // [pc, log_item]
                //     0x20 mstore    // [log_item]
                //     0x40 mstore    // []
                //     0x00 dup1      // [0x00, 0x00]
                //     0x60 dup2      // [0x00, 0x60, 0x00, 0x00]
                //     0x00000000000000000000000000000000bEefbabe
                //     gas            // [gas, beef_babe, 0x00, 0x60, 0x00, 0x00]
                //     staticcall pop // []
                // }

                // Check if we have exactly one 32 byte input
                if call.input.len() != 96 {
                    return (Return::Revert, gas, retdata)
                }

                let log_item = hex::encode(&call.input[64..96]);
                self.logs.push((pc, log_item));
            }
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
