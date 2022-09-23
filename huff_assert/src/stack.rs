use crate::{
    errors::{AssertError, ErrorKind},
    utils::format_arr,
    U256,
};
use huff_utils::{ast::AstSpan, bytecode::AssertSpan, prelude::MacroDefinition};
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
    pc_to_i_map: BTreeMap<usize, AssertSpan>,
    /// last operation
    last: usize,
    /// errors returned from inspection
    pub errors: Vec<AssertError>,
    /// macro to be inspected
    m: MacroDefinition,
    /// cache of assertions values
    cached: BTreeMap<String, U256>,
    /// stack state on macro execution
    stack: Vec<U256>,
}

impl StackInspector {
    pub fn new(
        pc_to_i_map: BTreeMap<usize, AssertSpan>,
        last: usize,
        m: MacroDefinition,
        stack: Vec<U256>,
    ) -> Self {
        Self { pc_to_i_map, last, errors: vec![], m, cached: BTreeMap::new(), stack }
    }

    /// Check assertion length and value at pc. Please note that stack is sent inverted for clarity
    fn check_assertion(&mut self, assertions: Vec<String>, stack: Vec<U256>, pc: usize) {
        let spans = self.pc_to_span(pc);

        if assertions.len() != stack.len() {
            let err = AssertError {
                kind: ErrorKind::Amount,
                expected: format_arr(assertions),
                got: format_arr(stack),
                spans,
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

    fn pc_to_span(&self, pc: usize) -> Option<AstSpan> {
        if let Some(assert) = self.pc_to_i_map.get(&pc) {
            let span = assert.clone().span;
            Some(span)
        } else {
            None
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
        for val in self.stack.iter() {
            interp.stack.push(*val).expect("Failed to push to stack");
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
                    spans: self.pc_to_span(pc),
                };

                self.errors.push(err);
            }

            if let Some(assert) = self.pc_to_i_map.get(&0_usize) {
                StackInspector::check_assertion(self, assert.clone().assertions, stack, 0);
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
                    spans: self.pc_to_span(pc),
                };

                self.errors.push(err);
            }
        }

        if let Some(assert) = self.pc_to_i_map.get(&pc) {
            self.check_assertion(assert.clone().assertions, stack, pc);
        }

        Return::Continue
    }
}
