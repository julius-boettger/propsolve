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
    Xor(Box<Ast<'src>>, Box<Ast<'src>>),

    Imp(Box<Ast<'src>>, Box<Ast<'src>>),

    Eq(Box<Ast<'src>>, Box<Ast<'src>>),
    Neq(Box<Ast<'src>>, Box<Ast<'src>>),
}

pub fn parse<'src>(input: String) -> Ast<'src> {
    println!("{:?}", parser().parse("a&b|c->d==e!=f"));
    std::process::exit(0) // DEBUG
}

fn parser<'src>() -> impl Parser<'src, &'src str, Ast<'src>> {
    let pad_char_op = |c: char| just(c).padded();
    recursive(|ast| {
        let atom = text::ident().map(|id| match id {
            "true" => Ast::Const(true),
            "false" => Ast::Const(false),
            _ => Ast::Var(id),
        }).padded();

        let nested_atom = choice((
            atom,
            ast.delimited_by(pad_char_op('('), pad_char_op(')')),
        ));

        let negation = pad_char_op('!')
            .repeated()
            .foldr(nested_atom, |_, rhs| Ast::Neg(Box::new(rhs)));

        let and_or_xor = negation.clone().foldl(
            choice((
                pad_char_op('&').to(Ast::And as fn(_, _) -> _),
                pad_char_op('|').to(Ast::Or as fn(_, _) -> _),
                pad_char_op('^').to(Ast::Xor as fn(_, _) -> _),
            ))
            .then(negation)
            .repeated(),
            |lhs, (operator, rhs)| operator(Box::new(lhs), Box::new(rhs)),
        );

        let implication = and_or_xor.clone().foldl(
            just("->").padded().to(Ast::Imp as fn(_, _) -> _)
            .then(and_or_xor)
            .repeated(),
            |lhs, (operator, rhs)| operator(Box::new(lhs), Box::new(rhs)),
        );

        implication.clone().foldl(
            choice((
                just("==").padded().to(Ast::Eq as fn(_, _) -> _),
                just("!=").padded().to(Ast::Neq as fn(_, _) -> _),
            ))
            .then(implication)
            .repeated(),
            |lhs, (operator, rhs)| operator(Box::new(lhs), Box::new(rhs)),
        )
    })
}
