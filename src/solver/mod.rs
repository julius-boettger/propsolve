use std::collections::HashSet;
use crate::{output::Output, parser::Ast};

fn get_identifiers<'src>(ast: &Ast<'src>) -> Vec<&'src str> {
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

    let mut identifiers: Vec<_> = identifiers.into_iter().collect();
    identifiers.sort_unstable();
    identifiers
}

pub fn solve<'src>(ast: &'src Ast<'src>) -> Output<'src> {
    println!("{ast:?}");

    let identifiers = get_identifiers(ast);
    Output::String(format!("identifiers: {identifiers:?}"))
}
