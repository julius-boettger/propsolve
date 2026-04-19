use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Easily describe and solve propositional formulas")]
struct Args {
    /// Path to the source file
    #[arg(required_unless_present = "expression")]
    file_path: Option<std::path::PathBuf>,

    /// Directly supply expression instead of reading from file
    #[arg(short, long, conflicts_with = "file_path")]
    expression: Option<String>,
}

pub fn get_input() -> String {
    let args = Args::parse();

    if let Some(file_path) = args.file_path {
        todo!()
    } else if let Some(expression) = args.expression {
        expression
    } else {
        unreachable!()
    }
}
