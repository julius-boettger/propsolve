pub enum Output<'src> {
    String(String),
    Model(Vec<(&'src str, bool)>),
}

pub fn output(output: &Output) {
    match output {
        Output::String(s) => println!("{s}"),
        Output::Model(model) => todo!(),
    }
}
