use crate::{parser::Ast, output::Output};

mod input;
mod output;
mod parser;
mod solver;

fn main() {
    let input = input::get_input();    
    let ast = parser::parse(&input);
    let result = solver::solve(&ast);
    output::output(&result);

    if let Output::Sat(_) = result {
        let neg_ast = Ast::Neg(Box::new(ast));
        let result = solver::solve(&neg_ast);
        output::output_neg_run(&result);
    }
}
