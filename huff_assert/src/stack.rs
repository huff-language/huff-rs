use crate::{
    errors::{AssertError, ErrorKind},
    utils::format_arr,
    U256,
};
use huff_utils::prelude::MacroDefinition;
use revm::{Database, EVMData, Inspector, Interpreter, Return};
use std::{
    collections::{
        btree_map::Entry::{Occupied, Vacant},
        BTreeMap,
    },
    fmt::Debug,
};
use tracing::span;

#[derive(Debug)]
pub struct StackInspector {
    /// pc to stack assertion
    pc_to_i_map: BTreeMap<usize, Vec<String>>,
    /// last operation
    last: usize,
    /// errors returned from inspection
    pub errors: Vec<AssertError>,
    /// macro to be inspected
    m: MacroDefinition,
    /// cache of assertions values
    cached: BTreeMap<String, U256>,
    /// stack state on macro execution
    stack: Option<Vec<U256>>,
}

impl StackInspector {
    pub fn new(
        pc_to_i_map: BTreeMap<usize, Vec<String>>,
        last: usize,
        m: MacroDefinition,
        stack: Option<Vec<U256>>,
    ) -> Self {
        Self { pc_to_i_map, last, errors: vec![], m, cached: BTreeMap::new(), stack }
    }

    /// Check assertion length and value at pc. Please note that stack is sent inverted for clarity
    fn check_assertion(&mut self, assertions: Vec<String>, stack: Vec<U256>, pc: usize) {
        let spans = if let Some(stat) = self.m.statements.get(pc) {
            let span = stat.clone().span;
            Some(span)
        } else {
            None
        };

        if assertions.len() != stack.len() {
            let err = AssertError {
                kind: ErrorKind::Amount,
                expected: format_arr(assertions),
                got: format_arr(stack),
                spans: spans.clone(),
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
                                spans: spans.clone(),
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
        if let Some(stack) = &self.stack {
            for val in stack.iter() {
                interp.stack.push(*val).expect("Failed to push to stack");
            }
        } else {
            // push 0 if not custom
            for _ in 0..self.m.takes {
                interp.stack.push(U256::zero()).expect("Failed to push to stack");
            }
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
                    spans: None,
                };

                self.errors.push(err);
            }

            if let Some(assertions) = self.pc_to_i_map.get(&(0 as usize)) {
                StackInspector::check_assertion(self, assertions.clone(), stack.clone(), 0);
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
                    spans: None,
                };

                self.errors.push(err);
            }
        }

        if let Some(assertions) = self.pc_to_i_map.get(&pc) {
            self.check_assertion(assertions.clone(), stack.clone(), pc);
        }

        Return::Continue
    }
}
