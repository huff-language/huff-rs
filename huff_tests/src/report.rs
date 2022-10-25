use crate::prelude::{ReportKind, TestResult, TestStatus};
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Attribute, Cell, Color, ContentArrangement,
    Row, Table,
};
use std::time::Instant;
use yansi::Paint;

/// Print a report of the test results, formatted according to the `report_kind` parameter.
pub fn print_test_report(results: Vec<TestResult>, report_kind: ReportKind, start: Instant) {
    // Gather how many of our tests passed *before* generating our report,
    // as we pass ownership of `results` to both the `ReportKind::Table`
    // and `ReportKind::List` arms.
    let n_passed = results
        .iter()
        .filter(|r| {
            std::mem::discriminant(&r.status) == std::mem::discriminant(&TestStatus::Success)
        })
        .count();
    let n_results = results.len();

    // Generate and print a report of the test results, formatted based on
    // the `report_kind` input.
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

            println!("{table}");
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

                let num_logs = result.logs.len().saturating_sub(1);

                if let Some(return_data) = result.return_data {
                    println!("├─ {}", Paint::cyan("RETURN DATA"));
                    println!("{} {return_data}", if num_logs == 0 { "╰─" } else { "├─" });
                }

                if num_logs > 0 {
                    println!("├─ {}", Paint::cyan("LOGS"));
                    result.logs.iter().enumerate().for_each(|(i, (pc, log))| {
                        let log = format!(
                            "[{}: {}]: 0x{}",
                            Paint::magenta("PC"),
                            Paint::yellow(pc),
                            log,
                        );
                        println!("{} {log}", if i == num_logs { "╰─" } else { "├─" });
                        // ├╌
                    });
                }
            }
        }
        ReportKind::JSON => {
            if let Ok(o) = serde_json::to_string_pretty(&results) {
                println!("{o}");
            } else {
                eprintln!("Error serializing test results into JSON.");
            }
            return
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
