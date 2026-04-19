use colored::Colorize;

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

pub fn output(output: &Output) {
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

pub fn output_neg_run(output: &Output) {
    println!();
    match output {
        Output::Sat(assignment) => {
            println!("{}, e.g. with:\n", format!("formula is also {}", "unsatisfiable".red()).bold());
            print_assignment(assignment);
        },
        Output::Unsat => println!("{}, meaning it is always satisfied", format!("formula is a {}", "tautology".green()).bold()),
        Output::Unknown => println!("could not solve negated formula (result {}), blame z3 🤷", "unknown".red()),
    }
}
