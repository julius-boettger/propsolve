use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::error::Rich;
use colored::Colorize;

use crate::input::Input;

pub enum Output {
    Sat(Vec<(String, bool)>),
    Unsat,
    Unknown,
}

fn print_assignment(assignment: &[(String, bool)]) {
    let mut assignment: Vec<_> = assignment.to_vec();
    assignment.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    for (identifier, value) in assignment {
        let value = if value { "1".green() } else { "0".red() };
        println!("{identifier} {} {value}", ":=".bright_black());
    }
}

pub fn output_first_run(output: &Output) {
    match output {
        Output::Sat(assignment) => {
            print!("{}", format!("formula is {}", "satisfiable".green()).bold());
            if !assignment.is_empty() {
                println!(", e.g. with:\n");
                print_assignment(assignment);
            }
        },
        Output::Unsat => println!("formula is {}", "unsatisfiable".red()),
        Output::Unknown => println!("could not solve formula (result {}), blame z3 🤷", "unknown".red()),
    }
}

pub fn output_both_runs(output: &Output, neg_output: &Output) {
    match neg_output {
        Output::Sat(assignment) => {
            output_first_run(output);
            println!("\n{}, e.g. with:\n", format!("formula is also {}", "unsatisfiable".red()).bold());
            print_assignment(assignment);
        },
        Output::Unsat => println!("{}, meaning it is always satisfied", format!("formula is a {}", "tautology".green()).bold()),
        Output::Unknown => println!("could not solve negated formula (result {}), blame z3 🤷", "unknown".red()),
    }
}

pub fn rich_parser_error(error: &Rich<'_, char>, input: &Input) {
    let source = &input.source;
    let span = (source, error.span().into_range());
    Report::build(ReportKind::Error, span.clone())
        .with_message(error.to_string())
        .with_label(
            Label::new(span)
                .with_message(error.reason().to_string())
                .with_color(Color::Red),
        )
        .finish()
        .print((source, Source::from(&input.expression)))
        .unwrap();
}

#[allow(clippy::unnecessary_debug_formatting)] // is necessary
pub fn invalid_path(file_path: &std::path::PathBuf) {
    println!("{} Path to input file ({file_path:?}) does not exist", "Error:".red().bold());
}

#[allow(clippy::unnecessary_debug_formatting)] // is necessary
pub fn path_is_dir(file_path: &std::path::PathBuf) {
    println!("{} Path to input file ({file_path:?}) is a directory, not a file", "Error:".red().bold());
}

#[allow(clippy::unnecessary_debug_formatting)] // is necessary
pub fn error_reading_path(file_path: &std::path::PathBuf, error: &std::io::Error) {
    println!("{} Reading path to input file ({file_path:?}) failed: {error}", "Error:".red().bold());
}

pub fn smt_string(smt_string: &str) {
    println!("{smt_string}");
}
