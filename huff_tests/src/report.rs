use crate::{runner::TestStatus, TestResult};
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Attribute, Cell, Color, ContentArrangement,
    Row, Table,
};
use std::time::Instant;
use yansi::Paint;

/// A test report kind
pub enum ReportKind {
    /// Signals `gen_report` to format the test report as a table
    Table,
    /// Signals `gen_report` to format the test report as a list
    List,
    /// Signals `gen_report` to format the test report as JSON
    JSON,
}

/// Convert a shared reference to an `Option<String>` to a `ReportKind`.
/// If the `Option<String>` is `None` or does not match any of the
/// `ReportKind` variants, then `ReportKind::List` is returned.
impl From<&Option<String>> for ReportKind {
    fn from(str: &Option<String>) -> Self {
        if let Some(str) = str {
            match str.to_lowercase().as_str() {
                "table" => ReportKind::Table,
                "list" => ReportKind::List,
                "json" => ReportKind::JSON,
                _ => panic!("Invalid report kind"),
            }
        } else {
            ReportKind::List
        }
    }
}

/// Print a report of the test results, formatted according to the `report_kind` parameter.
pub fn print_test_report(results: Vec<TestResult>, report_kind: ReportKind, start: Instant) {
    let n_passed = results
        .iter()
        .filter(|r| {
            std::mem::discriminant(&r.status) == std::mem::discriminant(&TestStatus::Success)
        })
        .count();
    let n_results = results.len();

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
        ReportKind::List => {
            for result in results {
                println!(
                    "[{0}] {1: <15} - {2} {3: <20}",
                    String::from(result.status),
                    result.name,
                    Paint::yellow("Gas used:"),
                    result.gas
                );
            }
        }
        ReportKind::JSON => {
            todo!()
        }
    }
    println!(
        "➜ {} tests passed, {} tests failed ({}%). ⏱ : {}",
        Paint::green(n_passed),
        Paint::red(n_results - n_passed),
        Paint::yellow(n_passed * 100 / n_results),
        Paint::magenta(format!("{:.4?}", start.elapsed()))
    );
}
