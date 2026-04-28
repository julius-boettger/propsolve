use chumsky::prelude::*;
use crate::{input, output};

#[derive(Debug, Clone)]
pub enum Ast<'src> {
    // atoms
    Const(bool),
    Var(&'src str),

    // operators

    Neg(Box<Self>),

    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Xor(Box<Self>, Box<Self>),

    Imp(Box<Self>, Box<Self>),

    Eq(Box<Self>, Box<Self>),
    Neq(Box<Self>, Box<Self>),
}

pub fn parse(input: &input::Input) -> Ast<'_> {
    parser().parse(&input.formula).into_result().unwrap_or_else(|errors| {
        for error in errors {
            output::rich_parser_error(&error, input);
        }
        std::process::exit(1)
    })
}

fn parser<'src>() -> impl Parser<'src, &'src str, Ast<'src>, extra::Err<Rich<'src, char>>> {
    let padding = choice((
        // single line comment
        just("//")
            .then(none_of('\n').repeated())
            .ignored(),
        // multi line comment
        just("/*")
            .then(any().and_is(just("*/").not()).repeated())
            .then(just("*/"))
            .ignored(),
        // other whitespace
        text::whitespace()
            .at_least(1) // to not conflict with other choices
            .ignored(),
    )).repeated();

    let atom = text::ident().map(|id| match id {
        "true" => Ast::Const(true),
        "false" => Ast::Const(false),
        _ => Ast::Var(id),
    }).padded_by(padding);

    let pad_char_op = |c: char| just(c).padded_by(padding);

    recursive(|ast| {
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
            just("->").padded_by(padding).to(Ast::Imp as fn(_, _) -> _)
            .then(and_or_xor)
            .repeated(),
            |lhs, (operator, rhs)| operator(Box::new(lhs), Box::new(rhs)),
        );

        let equality = implication.clone().foldl(
            choice((
                just("==").padded_by(padding).to(Ast::Eq as fn(_, _) -> _),
                just("!=").padded_by(padding).to(Ast::Neq as fn(_, _) -> _),
            ))
            .then(implication)
            .repeated(),
            |lhs, (operator, rhs)| operator(Box::new(lhs), Box::new(rhs)),
        );

        // all formulas separated by semicolons have to be true at the same time
        // => treat them as an AND of formulas
        let semicolons = equality.clone().foldl(
            pad_char_op(';').repeated().to(Ast::And as fn(_, _) -> _)
            .then(equality)
            .repeated(),
            |lhs, (operator, rhs)| operator(Box::new(lhs), Box::new(rhs)),
        );

        // ignore extra semicolons at the end
        semicolons.foldl(
            pad_char_op(';').ignored().repeated(),
            |lhs, ()| lhs
        )
        // speed up compile times a lot
        .boxed()
    })
}
