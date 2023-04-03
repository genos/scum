use crate::expression::{Atom, Expression, Identifier};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;
use smol_str::SmolStr;
use std::boxed::Box;

#[derive(Debug, thiserror::Error)]
pub enum ReadingError {
    #[error("Empty read")]
    Empty,
    #[error("Expected to parse into two pairs, got only one")]
    TooShort,
    #[error("Invalid read: expected one expression, got {0}")]
    Invalid(usize),
    #[error("Expected an expression to parse, but got {0:?}")]
    UnexpectedRule(Rule),
    #[error("Unable to parse float: {0:#?}")]
    ParsingFloat(#[from] std::num::ParseFloatError),
    #[error("Unable to parse int: {0:#?}")]
    ParsingInt(#[from] std::num::ParseIntError),
    #[error("Unexpected compound rule in atomic statement: {0:?}")]
    CompoundInAtom(Rule),
    #[error("Unable to parse atom: expeted 1 expression, found {0}")]
    BadAtom(usize),
    #[error("Unable to parse define: expected 2 expressions, found {0}")]
    BadDefine(usize),
    #[error("Unable to parse if statment: expected 3 expressions, found {0}")]
    BadIf(usize),
    #[error("Reading error: {location:?}, {line_col:?} {line}")]
    Other {
        location: pest::error::InputLocation,
        line_col: pest::error::LineColLocation,
        line: String,
    },
}

impl From<pest::error::Error<Rule>> for ReadingError {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Self::Other {
            location: e.clone().location,
            line_col: e.clone().line_col,
            line: e.line().to_string(),
        }
    }
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ScumParser;

pub(crate) fn read(input: &str) -> Result<Expression, ReadingError> {
    let mut pairs = ScumParser::parse(Rule::input, input)?.collect::<Vec<_>>();
    match pairs.len() {
        0 => Err(ReadingError::Empty),
        1 => Err(ReadingError::TooShort),
        n if n > 2 => Err(ReadingError::Invalid(n - 1)),
        _ => read_impl(pairs.remove(0).into_inner()),
    }
}

fn read_impl(pairs: Pairs<Rule>) -> Result<Expression, ReadingError> {
    let mut xs = vec![];
    for x in pairs {
        match x.as_rule() {
            Rule::constant => xs.push(constant_to_expr(x.into_inner())?),
            Rule::define => xs.push(define_to_expr(x.into_inner())?),
            Rule::ifte => xs.push(ifte_to_expr(x.into_inner())?),
            Rule::list => xs.push(list_to_expr(x.into_inner())?),
            r => return Err(ReadingError::UnexpectedRule(r)),
        }
    }
    if xs.len() == 1 {
        Ok(xs.remove(0))
    } else {
        Ok(Expression::List(xs))
    }
}

fn constant_to_expr(pairs: Pairs<Rule>) -> Result<Expression, ReadingError> {
    let mut pieces = pairs.collect::<Vec<_>>();
    if pieces.len() != 1 {
        Err(ReadingError::BadAtom(pieces.len()))
    } else {
        let pair = pieces.remove(0);
        match pair.as_rule() {
            Rule::bool => Ok(Expression::Constant(Atom::Bool(pair.as_str() == "#t"))),
            Rule::int => Ok(Expression::Constant(Atom::Int(
                pair.as_str().parse::<i64>()?,
            ))),
            Rule::float => Ok(Expression::Constant(Atom::Float(
                pair.as_str().parse::<f64>()?,
            ))),
            Rule::str => Ok(Expression::Constant(Atom::Str(pair.as_str().to_string()))),
            Rule::symbol => Ok(Expression::Constant(Atom::Symbol(Identifier(
                SmolStr::new(pair.as_str()),
            )))),
            r => Err(ReadingError::CompoundInAtom(r)),
        }
    }
}

fn define_to_expr(pairs: Pairs<Rule>) -> Result<Expression, ReadingError> {
    let mut pieces = pairs.collect::<Vec<_>>();
    if pieces.len() != 2 {
        Err(ReadingError::BadDefine(pieces.len()))
    } else {
        let key = read_impl(single(pieces.remove(0)))?;
        let value = read_impl(single(pieces.remove(0)))?;
        Ok(Expression::Define(Box::new(key), Box::new(value)))
    }
}

fn ifte_to_expr(pairs: Pairs<Rule>) -> Result<Expression, ReadingError> {
    let mut pieces = pairs.collect::<Vec<_>>();
    if pieces.len() != 3 {
        Err(ReadingError::BadIf(pieces.len()))
    } else {
        let cond = read_impl(single(pieces.remove(0)))?;
        let x = read_impl(single(pieces.remove(0)))?;
        let y = read_impl(single(pieces.remove(0)))?;
        Ok(Expression::If(Box::new(cond), Box::new(x), Box::new(y)))
    }
}

fn list_to_expr(pairs: Pairs<Rule>) -> Result<Expression, ReadingError> {
    let mut xs = vec![];
    for pair in pairs {
        xs.push(read_impl(single(pair))?);
    }
    Ok(Expression::List(xs))
}

fn single(p: Pair<Rule>) -> Pairs<Rule> {
    Pairs::single(p.into_inner().next().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    fn arb_identifier() -> impl Strategy<Value = Identifier> {
        // should match the identifier rule in grammar.pest
        r"([a-zA-Z!%&*/:<=>?^_~][a-zA-Z0-9!%&*/:<=>?^_~+[-].@]*)|[+-]"
            .prop_map(|s| Identifier(SmolStr::new(&s)))
    }

    fn arb_atom() -> impl Strategy<Value = Atom> {
        prop_oneof![
            any::<bool>().prop_map(Atom::Bool),
            any::<f64>().prop_map(Atom::Float),
            any::<i64>().prop_map(Atom::Int),
            r#""[\w\s\d\u{7f}]*""#.prop_map(Atom::Str),
            arb_identifier().prop_map(Atom::Symbol),
        ]
    }

    fn arb_expression() -> impl Strategy<Value = Expression> {
        // https://docs.rs/proptest/latest/proptest/prelude/trait.Strategy.html#method.prop_recursive
        arb_atom().prop_map(Expression::Constant).prop_recursive(
            4,  // No more than 4 branch levels deep
            64, // Target around 64 total elements
            16, // Each collection is up to 16 elements long
            |atom| {
                prop_oneof![
                    (atom.clone(), atom.clone()).prop_map(|(key, value)| Expression::Define(
                        Box::new(key),
                        Box::new(value)
                    )),
                    (atom.clone(), atom.clone(), atom.clone()).prop_map(|(cond, x, y)| {
                        Expression::If(Box::new(cond), Box::new(x), Box::new(y))
                    }),
                    prop::collection::vec(atom, 0..16).prop_map(Expression::List),
                ]
            },
        )
    }

    proptest! {
        #[test]
        fn atom_round_trip(atom in arb_atom()) {
            let c = Expression::Constant(atom);
            let s = c.to_string();
            let p = read(&s);
            prop_assert!(p.is_ok());
            prop_assert_eq!(p.unwrap(), c);
        }

        #[test]
        fn expression_round_trip(exp in arb_expression()) {
            let s = exp.to_string();
            let p = read(&s);
            prop_assert!(p.is_ok());
            prop_assert_eq!(p.unwrap(), exp);
        }
    }
}