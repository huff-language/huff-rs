use comfy_table::{Cell, Color};
use serde::Serialize;
use yansi::Paint;

/// A test result
#[derive(Debug, Clone, Serialize)]
pub struct TestResult {
    pub name: String,
    pub return_data: Option<String>,
    pub gas: u64,
    pub status: TestStatus,
}

/// A test status variant
#[derive(Debug, Clone, Serialize)]
pub enum TestStatus {
    Success,
    Revert,
}

/// Convert a TestStatus variant directly to a colored string for use in the report
impl From<TestStatus> for String {
    fn from(status: TestStatus) -> Self {
        match status {
            TestStatus::Success => Paint::green("PASS").to_string(),
            TestStatus::Revert => Paint::red("FAIL").to_string(),
        }
    }
}

/// Convert a TestStatus variant directly to a table cell for use in the report
impl From<TestStatus> for Cell {
    fn from(status: TestStatus) -> Self {
        match status {
            TestStatus::Success => Cell::new("PASS").fg(Color::Green),
            TestStatus::Revert => Cell::new("FAIL").fg(Color::Red),
        }
    }
}
