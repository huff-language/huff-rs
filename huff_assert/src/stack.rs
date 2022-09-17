use crate::{
    errors::{AssertError, ErrorKind},
    U256,
};
use huff_utils::{bytecode::Bytes as HuffBytes, prelude::MacroDefinition};
use revm::{Database, EVMData, Inspector, Interpreter, Return};
use std::{
    collections::{
        btree_map::Entry::{Occupied, Vacant},
        BTreeMap,
    },
    fmt::Debug,
};

#[derive(Debug)]
pub struct StackInspector {
    /// pc to stack assertion
    pc_to_i_map: BTreeMap<usize, HuffBytes>,
    /// last operation
    last: usize,
    /// errors returned from inspection
    pub errors: Vec<AssertError>,
    /// macro to be inspected
    m: MacroDefinition,
    /// cache of assertions values
    cached: BTreeMap<String, U256>,
}

impl StackInspector {
    pub fn new(pc_to_i_map: BTreeMap<usize, HuffBytes>, last: usize, m: MacroDefinition) -> Self {
        Self { pc_to_i_map, last, errors: vec![], m, cached: BTreeMap::new() }
    }

    /// Check assertion length and value at pc. Please note that stack is sent inverted for clarity
    fn check_assertion(&mut self, assertions: Vec<String>, stack: Vec<U256>) {
        if assertions.len() != stack.len() {
            let err = AssertError {
                kind: ErrorKind::Amount,
                expected: assertions.into_iter().collect::<String>().clone(),
                got: stack.into_iter().map(|n| n.to_string()).collect::<String>().clone(),
            };

            self.errors.push(err);
        } else {
            for (i, a) in assertions.into_iter().enumerate() {
                match self.cached.entry(a) {
                    Vacant(entry) => {
                        entry.insert(*stack.get(i).expect("No valid index"));
                    }
                    Occupied(entry) => {
                        let stack_val = stack.get(i).expect("No valid index");
                        if entry.get() != stack_val {
                            let err = AssertError {
                                kind: ErrorKind::Value,
                                expected: format!("`{}` == `{}`", entry.key(), stack_val),
                                got: format!("`{}`", entry.get()),
                            };

                            self.errors.push(err);
                        }
                    }
                }
            }
        }
    }
}

impl<DB: Database + Debug> Inspector<DB> for StackInspector {
    fn initialize_interp(
        &mut self,
        interp: &mut Interpreter,
        _data: &mut EVMData<'_, DB>,
        _is_static: bool,
    ) -> Return {
        for _ in 0..self.m.takes {
            interp.stack.push(U256::zero()).expect("Failed to push to stack");
        }

        Return::Continue
    }

    fn step(
        &mut self,
        interp: &mut Interpreter,
        _data: &mut EVMData<'_, DB>,
        _is_static: bool,
    ) -> Return {
        let pc = interp.program_counter();
        let stack = interp.stack().data().clone();
        let stack = stack.into_iter().rev().collect::<Vec<U256>>();

        // Check at macro start
        if pc == 0 {
            if stack.len() != self.m.takes {
                let err = AssertError {
                    kind: ErrorKind::Takes,
                    expected: format!("`takes({})`", self.m.takes),
                    got: format!("`{:?}`", stack),
                };

                self.errors.push(err);
            }

            if let Some(assertions) = self.pc_to_i_map.get(&(0 as usize)) {
                let assertions =
                    assertions.0.split(',').map(|s| s.to_string()).collect::<Vec<String>>();

                StackInspector::check_assertion(self, assertions, stack.clone());
            }
        }

        Return::Continue
    }

    fn step_end(
        &mut self,
        interp: &mut Interpreter,
        _data: &mut EVMData<'_, DB>,
        _is_static: bool,
        _eval: Return,
    ) -> Return {
        let pc = interp.program_counter();
        let stack = interp.stack().data().clone();
        let stack = stack.into_iter().rev().collect::<Vec<U256>>();

        if pc == self.last {
            // is at returns
            if stack.len() != self.m.returns {
                let err = AssertError {
                    kind: ErrorKind::Returns,
                    expected: format!("`returns({})`", self.m.returns),
                    got: format!("`{:?}`", stack),
                };

                self.errors.push(err);
            }
        }

        if let Some(assertions) = self.pc_to_i_map.get(&pc) {
            let assertions =
                assertions.0.split(',').map(|s| s.to_string()).collect::<Vec<String>>();

            StackInspector::check_assertion(self, assertions, stack.clone());
        }

        Return::Continue
    }
}
