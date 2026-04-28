use crate::{parser::Ast, output::Output};

mod input;
mod output;
mod parser;
mod solver;

fn main() {
    let input = input::get_input();    
    let ast = parser::parse(&input);
    let result = solver::solve(&input, &ast);

    if let Output::Sat(_) = result {
        let neg_ast = Ast::Neg(Box::new(ast));
        let neg_result = solver::solve(&input, &neg_ast);
        output::output_both_runs(&result, &neg_result);
    } else {
        output::output_first_run(&result);
    }
}
