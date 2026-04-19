use super::output;
use clap::Parser;

pub struct Input {
    pub expression: String,
    // e.g. the file path
    pub source: String,
}

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

pub fn get_input() -> Input {
    let args = Args::parse();

    if let Some(file_path) = args.file_path {
        if !file_path.exists() {
            output::invalid_path(&file_path);
            std::process::exit(1);
        }

        if file_path.is_dir() {
            output::path_is_dir(&file_path);
            std::process::exit(1);
        }

        match std::fs::read_to_string(file_path.clone()) {
            Ok(expression) => Input { expression, source: String::from(file_path.to_string_lossy()) },
            Err(error) => {
                output::error_reading_path(&file_path, &error);
                std::process::exit(1);
            },
        }
    } else if let Some(expression) = args.expression {
        Input { expression, source: String::from("expression") }
    } else {
        unreachable!()
    }
}
