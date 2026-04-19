pub enum Output {
    Sat(Vec<(String, bool)>),
    Unsat,
    Unknown,
}

pub fn output(output: &Output) {
    match output {
        Output::Sat(items) => println!("{items:?}"),
        Output::Unsat => todo!(),
        Output::Unknown => todo!(),
    }
}
