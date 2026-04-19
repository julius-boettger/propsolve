use colored::Colorize;

pub enum Output {
    Sat(Vec<(String, bool)>),
    Unsat,
    Unknown,
}

pub fn output(output: &Output) {
    match output {
        Output::Sat(assignment) => {
            let mut assignment: Vec<_> = assignment.clone();
            assignment.sort_unstable_by(|a, b| a.0.cmp(&b.0));
            println!("{}, e.g. with:", format!("formula is {}", "satisfiable".green()).bold());
            for (identifier, value) in assignment {
                let value = if value { "1".green() } else { "0".red() };
                println!("{identifier} {} {value}", ":=".bright_black());
            }
        },
        Output::Unsat => println!("formula is {}", "unsatisfiable".red()),
        Output::Unknown => println!("could not solve formula (result {}), blame z3 🤷", "unknown".red()),
    }
}
