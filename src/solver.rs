use std::{collections::{HashMap, HashSet}};
use z3::{SatResult, Solvable, Solver, ast::Bool};

use crate::{input::Input, output::{self, Output}, parser::Ast};

fn get_identifiers<'src>(ast: &Ast<'src>) -> HashSet<&'src str> {
    fn collect_identifiers<'src>(ast: &Ast<'src>, identifiers: &mut HashSet<&'src str>) {
        match ast {
            Ast::Var(id) => { identifiers.insert(id); },
            Ast::Const(_) => {},

            Ast::Neg(inner) => collect_identifiers(inner, identifiers),

            Ast::And(l, r)
            | Ast::Or(l, r)
            | Ast::Xor(l, r)
            | Ast::Imp(l, r)
            | Ast::Eq(l, r)
            | Ast::Neq(l, r) => {
                collect_identifiers(l, identifiers);
                collect_identifiers(r, identifiers);
            }
        }
    }

    let mut identifiers = HashSet::new();
    collect_identifiers(ast, &mut identifiers);
    identifiers
}

fn build_formula<'src>(ast: &'src Ast<'src>, identifier_map: &HashMap<&'src str, Bool>) -> Bool {
    match ast {
        Ast::Const(value) => Bool::from_bool(*value),
        Ast::Var(id) => identifier_map.get(id).unwrap().clone(),
        Ast::Neg(ast) => build_formula(ast, identifier_map).not(),
        Ast::And(lhs, rhs) => build_formula(lhs, identifier_map) & build_formula(rhs, identifier_map),
        Ast::Or(lhs, rhs) => build_formula(lhs, identifier_map) | build_formula(rhs, identifier_map),
        Ast::Xor(lhs, rhs) => build_formula(lhs, identifier_map) ^ build_formula(rhs, identifier_map),
        Ast::Imp(lhs, rhs) => build_formula(lhs, identifier_map).implies(build_formula(rhs, identifier_map)),
        Ast::Eq(lhs, rhs) => build_formula(lhs, identifier_map).eq(build_formula(rhs, identifier_map)),
        Ast::Neq(lhs, rhs) => build_formula(lhs, identifier_map).ne(build_formula(rhs, identifier_map)),
    }
}

pub fn solve<'src>(input: &Input, ast: &'src Ast<'src>) -> Output {
    let identifiers = get_identifiers(ast);
    let identifier_map: HashMap<_, _> = identifiers.iter()
        .map(|id| (*id, Bool::new_const(*id)))
        .collect();

    let formula = build_formula(ast, &identifier_map);
    let solver = Solver::new();
    solver.assert(formula);

    if input.print {
        let smt_string = solver.to_smt2();
        // first two lines contain a comment and a (set-info ...)
        let mut smt_string: String = smt_string.split('\n').skip(2)
            .collect::<Vec<_>>().join("\n");
        smt_string.push_str("(get-model)");

        output::smt_string(&smt_string);
        std::process::exit(0);
    }

    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();

            let assignment: Vec<_> = identifier_map.values().map(|variable| {
                // this needs to be here to not mess with own ast enum 
                use z3::ast::Ast;

                let identifier = variable.decl().name();
                let value = variable.read_from_model(&model, true).unwrap().as_bool().unwrap();

                (identifier, value)
            }).collect();

            Output::Sat(assignment)
        }
        SatResult::Unsat => Output::Unsat,
        SatResult::Unknown => Output::Unknown,
    }
}
