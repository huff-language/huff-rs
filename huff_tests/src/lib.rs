use crate::{
    errors::RunnerError,
    runner::{TestResult, TestRunner},
};
use huff_utils::prelude::{Contract, MacroDefinition};

/// The runner module
pub mod runner;

/// The inspector module
pub mod inspector;

/// The errors module
pub mod errors;

/// A vector of test macros
pub type TestMacros = Vec<MacroDefinition>;

/// A Huff Tester
pub struct HuffTester {
    /// The AST of the contract
    pub ast: Contract,

    /// The test macros
    pub macros: TestMacros,

    /// The test runner
    pub runner: TestRunner,
}

/// A test report kind
pub enum ReportKind {
    /// Signals `gen_report` to format the test report as a table
    Table,
    /// Signals `gen_report` to format the test report as JSON
    JSON,
}

/// HuffTester implementation
impl HuffTester {
    pub fn new(ast: Contract, macros: TestMacros) -> Self {
        Self { ast, macros, runner: TestRunner::default() }
    }

    /// Execute tests
    pub fn execute(&mut self) -> Result<Vec<TestResult>, RunnerError> {
        todo!()
    }

    /// Generate a report of the test results
    pub fn gen_report(&self, _results: Vec<TestResult>, _report_kind: ReportKind) {
        todo!()
    }
}
