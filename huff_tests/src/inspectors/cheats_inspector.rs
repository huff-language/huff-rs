use crate::cheats::{HuffCheatCode, HUFF_CHEATS_MAP};
use bytes::Bytes;
use ethers_core::{types::Address, utils::hex};
use lazy_static::lazy_static;
use revm::{
    interpreter::{CallInputs, CreateInputs, Gas, InstructionResult},
    primitives::B160,
    Database, EVMData, Inspector,
};
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
    fn log(
        &mut self,
        _: &mut EVMData<'_, DB>,
        _: &revm::primitives::B160,
        _: &[revm::primitives::B256],
        _: &Bytes,
    ) {
        unimplemented!()
    }

    fn call(
        &mut self,
        _data: &mut EVMData<'_, DB>,
        inputs: &mut CallInputs,
        _is_static: bool,
    ) -> (InstructionResult, Gas, Bytes) {
        (InstructionResult::Continue, Gas::new(inputs.gas_limit), Bytes::new())
    }

    fn call_end(
        &mut self,
        _data: &mut EVMData<'_, DB>,
        call: &CallInputs,
        remaining_gas: Gas,
        status: InstructionResult,
        out: Bytes,
        _is_static: bool,
    ) -> (InstructionResult, Gas, Bytes) {
        let revm_cheats = revm::primitives::B160::from_slice(CHEATS_ADDR.as_bytes());
        if call.contract == revm_cheats && call.input.len() >= 64 {
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
                    return (InstructionResult::Revert, remaining_gas, out)
                }

                let log_item = hex::encode(&call.input[64..96]);
                self.logs.push((pc, log_item));
            }
        }

        (status, remaining_gas, out)
    }

    fn create(
        &mut self,
        _data: &mut EVMData<'_, DB>,
        call: &mut CreateInputs,
    ) -> (InstructionResult, Option<B160>, Gas, Bytes) {
        (InstructionResult::Continue, None, Gas::new(call.gas_limit), Bytes::new())
    }

    fn create_end(
        &mut self,
        _data: &mut EVMData<'_, DB>,
        _inputs: &CreateInputs,
        ret: InstructionResult,
        address: Option<B160>,
        remaining_gas: Gas,
        out: Bytes,
    ) -> (InstructionResult, Option<B160>, Gas, Bytes) {
        (ret, address, remaining_gas, out)
    }
}
