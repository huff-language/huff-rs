use huff_utils::bytecode::Bytes as HuffBytes;
use revm::{Database, EVMData, Inspector, Interpreter, Return};
use std::collections::BTreeMap;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct StackInspector {
    pc_to_i_map: BTreeMap<usize, HuffBytes>,

    pub errors: Vec<String>,
}

impl StackInspector {
    pub fn new(pc_to_i_map: BTreeMap<usize, HuffBytes>) -> Self {
        Self { pc_to_i_map, errors: vec![] }
    }
}

impl<DB: Database + Debug> Inspector<DB> for StackInspector {
    fn step_end(
        &mut self,
        interp: &mut Interpreter,
        _data: &mut EVMData<'_, DB>,
        _is_static: bool,
        _eval: Return,
    ) -> Return {
        let pc = interp.program_counter();
        let stack = interp.stack().data();

        if let Some(assertions) = self.pc_to_i_map.get(&pc) {
            if let Some(assertions) = assertions.0.strip_prefix("stack: ") {
                let assertions = if assertions == " " {
                    // Is empty, might require a less hacky solution
                    vec![]
                } else {
                    let assertions: Vec<String> = assertions
                        .split(',')
                        .map(|a| a.split_whitespace().collect::<String>())
                        .collect();

                    assertions
                };

                // println!("{:?} {:?}", &stack, &assertions);

                if assertions.len() != stack.len() {
                    let err =
                        format!("wrong assertion: expected `{:?}` got `{:?}`", &assertions, &stack);

                    self.errors.push(err);
                }
            }
        }

        Return::Continue
    }
}
