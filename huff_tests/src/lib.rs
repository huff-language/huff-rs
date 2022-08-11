use crate::{errors::RunnerError, runner::TestRunner, types::TestResult};
use huff_utils::prelude::{Contract, MacroDefinition};
use std::{borrow::Borrow, rc::Rc};

/// The runner module
pub mod runner;

/// The inspector module
pub mod inspector;

/// The report module
pub mod report;

/// The types module
pub mod types;

/// The errors module
pub mod errors;

/// Prelude wraps all modules within the crate
pub mod prelude {
    pub use crate::{errors::*, inspector::*, report::*, runner::*, types::*};
}

/// A vector of shared references to test macro definitions
pub type TestMacros<'t> = Vec<&'t MacroDefinition>;

/// The core struct of the huff-tests crate.
///
/// A `HuffTester` struct is instantiated with an AST of a contract that contains test
/// macros. The struct can be consumed by the [HuffTester::execute](execute) method,
/// returning a vector of [TestResult](TestResult) structs.
pub struct HuffTester<'t> {
    /// The AST of the contract
    pub ast: &'t Contract,

    /// The test macros
    pub macros: TestMacros<'t>,

    /// The test runner
    pub runner: TestRunner,
}

/// HuffTester implementation
impl<'t> HuffTester<'t> {
    /// Create a new instance of `HuffTester` from a contract's AST.
    pub fn new(ast: &'t Contract, match_: Rc<Option<String>>) -> Self {
        Self {
            ast,
            macros: {
                let mut macros: TestMacros<'t> = ast.macros.iter().filter(|m| m.test).collect();
                if let Some(match_) = match_.borrow() {
                    macros.retain(|m| m.name == *match_);
                }
                macros
            },
            runner: TestRunner::default(),
        }
    }

    /// Execute tests
    pub fn execute(mut self) -> Result<Vec<TestResult>, RunnerError> {
        if self.macros.is_empty() {
            return Err(RunnerError(String::from("No test macros found.")))
        }

        self.macros
            .into_iter()
            .map(|macro_def| self.runner.run_test(macro_def, self.ast))
            .collect::<Result<Vec<TestResult>, RunnerError>>()
    }
}
