use crate::utils::{build_ic_pc_map, build_pc_ic_map, ICPCMap, PCICMap};
use bytes::Bytes;
use huff_utils::bytecode::Bytes as HuffBytes;
use revm::{Database, EVMData, Inspector, Interpreter, Return, SpecId, Stack};
use std::borrow::Borrow;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct StackInspector {
    // stack: Vec<Stack>
    pc_to_ic_map: PCICMap,
    ic_to_pc_map: ICPCMap,
    pc_to_i_map: BTreeMap<usize, Vec<HuffBytes>>,
}

impl StackInspector {
    pub fn new(code: &Bytes, pc_to_i_map: BTreeMap<usize, Vec<HuffBytes>>) -> Self {
        let pc_to_ic_map = build_pc_ic_map(SpecId::LATEST, code);
        let ic_to_pc_map = build_ic_pc_map(SpecId::LATEST, code);
        Self { pc_to_ic_map, ic_to_pc_map, pc_to_i_map }
    }
}

impl<DB: Database> Inspector<DB> for StackInspector {
    /*fn initialize_interp(
        &mut self,
        interp: &mut Interpreter,
        data: &mut EVMData<'_, DB>,
        _is_static: bool,
    ) -> Return {

        Return::Continue
    }*/

    // TODO: Make verification if has stack assertion right after
    fn step_end(
        &mut self,
        interp: &mut Interpreter,
        _data: &mut EVMData<'_, DB>,
        _is_static: bool,
        _eval: Return,
    ) -> Return {
        // dbg!(interp.stack().data(), interp.program_counter());
        let pc = interp.program_counter();
        dbg!(&pc);
        let ic = self.pc_to_ic_map[&pc];
        dbg!(&ic);

        /*let i = &self.pc_to_i_map[&pc];
        dbg!(&i);*/

        Return::Continue
    }
}
