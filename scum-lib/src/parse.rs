use crate::expression::{Atom, Expression, Identifier};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ScumParser;

#[derive(Debug, thiserror::Error)]
pub enum ParsingError {
    #[error("Unable to parse float: {0:#?}")]
    ParsingFloat(#[from] std::num::ParseFloatError),
    #[error("Unable to parse int: {0:#?}")]
    ParsingInt(#[from] std::num::ParseIntError),
    #[error("Parsing error: {location:?}, {line_col:?} {line}")]
    GeneralParsing {
        location: pest::error::InputLocation,
        line_col: pest::error::LineColLocation,
        line: String,
    },
}

impl From<pest::error::Error<Rule>> for ParsingError {
    fn from(e: pest::error::Error<Rule>) -> Self {
        Self::GeneralParsing {
            location: e.clone().location,
            line_col: e.clone().line_col,
            line: e.line().to_string(),
        }
    }
}

pub fn parse(input: &str) -> Result<Expression, ParsingError> {
    let mut xs = vec![];
    let pairs = ScumParser::parse(Rule::expression, input)?;
    for x in pairs {
        match x.as_rule() {
            Rule::constant => {
                for y in x.into_inner() {
                    match y.as_rule() {
                        Rule::bool => xs.push(Expression::Constant(Atom::Bool(y.as_str() == "#t"))),
                        Rule::int => {
                            xs.push(Expression::Constant(Atom::Int(y.as_str().parse::<i64>()?)))
                        }
                        Rule::float => xs.push(Expression::Constant(Atom::Float(
                            y.as_str().parse::<f64>()?,
                        ))),
                        Rule::string => xs.push(Expression::Constant(Atom::Str(
                            y.into_inner().next().unwrap().as_str().to_string(),
                        ))),
                        Rule::symbol => xs.push(Expression::Constant(Atom::Symbol(Identifier(
                            y.as_str().to_string(),
                        )))),
                        _ => {}
                    }
                }
            }
            Rule::list => {
                let mut ys = vec![];
                for y in x.into_inner() {
                    ys.push(parse(y.as_str())?);
                }
                xs.push(Expression::List(ys));
            }
            _ => {}
        }
    }
    if xs.len() == 1 {
        Ok(xs.pop().unwrap())
    } else {
        Ok(Expression::List(xs))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    fn arb_identifier() -> impl Strategy<Value = Identifier> {
        r"([+[-]*][^\s0-9\(\)]*)|([a-zA-Z][^\(\)\s]*)".prop_map(Identifier)
    }

    fn arb_atom() -> impl Strategy<Value = Atom> {
        prop_oneof![
            any::<bool>().prop_map(Atom::Bool),
            any::<f64>().prop_map(Atom::Float),
            any::<i64>().prop_map(Atom::Int),
            r"[\w\s\d\u{7f}]*".prop_map(Atom::Str),
            arb_identifier().prop_map(Atom::Symbol),
        ]
    }

    fn arb_expression() -> impl Strategy<Value = Expression> {
        // https://docs.rs/proptest/latest/proptest/prelude/trait.Strategy.html#method.prop_recursive
        arb_atom()
            .prop_map(Expression::Constant)
            .prop_recursive(
                4,  // No more than 4 branch levels deep
                64, // Target around 64 total elements
                16, // Each collection is up to 16 elements long
                |element| {
                    prop_oneof![
                        prop::collection::vec(element, 0..16).prop_map(Expression::List),
                    ]
                },
            )
    }

    proptest! {
        #[test]
        fn expr_round_trip(exp in arb_expression()) {
            let s = exp.to_string();
            let p = parse(&s);
            dbg!(&p);
            prop_assert!(p.is_ok());
            let exp2 = p.unwrap();
            prop_assert_eq!(exp2, exp);
        }
    }
}
