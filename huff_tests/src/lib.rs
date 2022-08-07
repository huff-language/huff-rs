use crate::{
    errors::RunnerError,
    runner::{TestResult, TestRunner},
};
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Attribute, Cell, Color, ContentArrangement,
    Row, Table,
};
use huff_utils::prelude::{Contract, MacroDefinition};

/// The runner module
pub mod runner;

/// The inspector module
pub mod inspector;

/// The errors module
pub mod errors;

/// A vector of test macros
pub type TestMacros<'t> = Vec<&'t MacroDefinition>;

/// A Huff Tester
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
    pub fn new(ast: &'t Contract) -> Self {
        Self {
            ast,
            macros: ast.macros.iter().filter(|m| m.test).collect(),
            runner: TestRunner::default(),
        }
    }

    /// Execute tests
    pub fn execute(mut self) -> Result<Vec<TestResult>, RunnerError> {
        self.macros
            .into_iter()
            .map(|macro_def| self.runner.run_test(macro_def, self.ast))
            .collect::<Result<Vec<TestResult>, RunnerError>>()
    }
}

/// A test report kind
pub enum ReportKind {
    /// Signals `gen_report` to format the test report as a table
    Table,
    /// Signals `gen_report` to format the test report as JSON
    JSON,
}

/// Generate a report of the test results
pub fn gen_test_report(results: Vec<TestResult>, report_kind: ReportKind) {
    match report_kind {
        ReportKind::Table => {
            let mut table = Table::new();
            table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);
            table.set_header(Row::from(vec![
                Cell::new("Name").fg(Color::Magenta),
                Cell::new("Return Data").fg(Color::Yellow),
                Cell::new("Gas").fg(Color::Cyan),
                Cell::new("Status").fg(Color::Blue),
            ]));
            table.set_content_arrangement(ContentArrangement::DynamicFullWidth);
            table.set_width(120);

            for result in results {
                table.add_row(Row::from(vec![
                    Cell::new(result.name).add_attribute(Attribute::Bold).fg(Color::Cyan),
                    Cell::new(result.return_data.unwrap_or_else(|| String::from("None"))),
                    Cell::new(result.gas.to_string()),
                    Cell::from(result.status),
                ]));
            }

            println!("{}", table);
        }
        ReportKind::JSON => {
            todo!()
        }
    }
}
