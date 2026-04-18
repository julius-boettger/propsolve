mod input;
mod output;
mod parser;
mod solver;

fn main() {
    let input = input::get_input();    
    let ast = parser::parse(&input);
    let result = solver::solve(&ast);
    output::output(&result);
}
