use chumsky::prelude::*;

#[derive(Debug, Clone)]
pub enum Ast<'src> {
    // atoms
    Const(bool),
    Var(&'src str),

    // operators
    Neg(Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    //Imp(Box<Ast<'src>>, Box<Ast<'src>>),
    //Xor(Box<Ast<'src>>, Box<Ast<'src>>),
    //Eq(Box<Ast<'src>>, Box<Ast<'src>>),
    //Neq(Box<Ast<'src>>, Box<Ast<'src>>),
}

pub fn parse<'src>(input: String) -> Ast<'src> {
    println!("{:?}", parser().parse("!(!(true))&!(false)"));
    std::process::exit(0) // DEBUG
}

fn parser<'src>() -> impl Parser<'src, &'src str, Ast<'src>> {
    let pad_op = |c| just(c).padded();
    recursive(|ast| {
        let atom = text::ident().map(|id| match id {
            "true" => Ast::Const(true),
            "false" => Ast::Const(false),
            _ => Ast::Var(id),
        }).padded();

        let nested_atom = choice((
            atom,
            ast.delimited_by(just('('), just(')')),
        ));

        let negation = pad_op('!')
            .repeated()
            .foldr(nested_atom, |_, rhs| Ast::Neg(Box::new(rhs)));

        negation.clone().foldl(
            choice((
                pad_op('&').to(Ast::And as fn(_, _) -> _),
                pad_op('|').to(Ast::Or as fn(_, _) -> _),
            ))
            .then(negation)
            .repeated(),
            |lhs, (operator, rhs)| operator(Box::new(lhs), Box::new(rhs)),
        )
    })
}
