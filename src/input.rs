use super::output;
use clap::Parser;
use std::io::{IsTerminal, Read};

pub struct Input {
    pub formula: String,
    // e.g. the file path
    pub source: String,
    // whether to print the formula in SMT-LIB
    pub print: bool,
}

#[derive(Parser)]
#[command(version, about = "Easily describe and solve propositional formulas")]
struct Args {
    /// Path to the formula source file
    #[arg(required_unless_present = "eval", required_unless_present = "stdin")]
    file_path: Option<std::path::PathBuf>,

    /// Directly supply formula to evaluate
    #[arg(short, long, conflicts_with = "file_path", conflicts_with = "stdin")]
    eval: Option<String>,

    /// Read formula from standard input (pipe)
    #[arg(short, long, conflicts_with = "file_path", conflicts_with = "eval")]
    stdin: bool,

    /// Print formula in SMT-LIB language instead of solving it
    #[arg(short, long)]
    print: bool,
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
            Ok(formula) => Input {
                formula,
                source: String::from(file_path.to_string_lossy()),
                print: args.print
            },
            Err(error) => {
                output::error_reading_path(&file_path, &error);
                std::process::exit(1);
            },
        }
    } else if args.stdin {
        let stdin = std::io::stdin();

        // check if there is any data
        if stdin.is_terminal() {
            output::no_stdin();
            std::process::exit(1);
        }

        let mut formula = String::new();
        stdin.lock().read_to_string(&mut formula).unwrap();
        Input { formula, source: String::from("stdin"), print: args.print }
    } else if let Some(formula) = args.eval {
        Input { formula, source: String::from("eval"), print: args.print }
    } else {
        // should be guaranteed by clap args
        unreachable!()
    }
}
