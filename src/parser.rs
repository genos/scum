use crate::expression::{Atom, Expression, Identifier};
use pest::{error::Error, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct ScumParser;

fn _expression(input: &str) -> Result<Expression, Error<Rule>> {
    let mut xs = vec![];
    let pairs = ScumParser::parse(Rule::expression, input)?;
    for x in pairs {
        match x.as_rule() {
            Rule::constant => {
                for y in x.into_inner() {
                    match y.as_rule() {
                        Rule::bool => xs.push(Expression::Constant(Atom::Bool(y.as_str() == "#t"))),
                        Rule::int => xs.push(Expression::Constant(Atom::Int(
                            y.as_str().parse::<i64>().unwrap(),
                        ))),
                        Rule::float => xs.push(Expression::Constant(Atom::Float(
                            y.as_str().parse::<f64>().unwrap(),
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
                    ys.push(_expression(y.as_str())?);
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
        "([+[-]*][^\\s0-9\\(\\)]*)|([a-zA-Z][^\\(\\)\\s]*)".prop_map(Identifier)
    }

    fn arb_atom() -> impl Strategy<Value = Atom> {
        prop_oneof![
            any::<bool>().prop_map(Atom::Bool),
            any::<f64>().prop_map(Atom::Float),
            any::<i64>().prop_map(Atom::Int),
            arb_identifier().prop_map(Atom::Symbol),
        ]
    }

    fn arb_expression() -> impl Strategy<Value = Expression> {
        // https://docs.rs/proptest/latest/proptest/prelude/trait.Strategy.html#method.prop_recursive
        arb_atom()
            .prop_map(|a| Expression::Constant(a))
            .prop_recursive(
                4,  // No more than 4 branch levels deep
                64, // Target around 64 total elements
                16, // Each collection is up to 16 elements long
                |element| {
                    prop_oneof![
                        prop::collection::vec(element.clone(), 0..16).prop_map(Expression::List),
                    ]
                },
            )
    }

    proptest! {
        #[test]
        fn arb_id_ok(i in arb_identifier()) {
            prop_assert_eq!(i.clone(), i.clone())
        }

        #[test]
        fn arb_atom_ok(a in arb_atom()) {
            prop_assert_eq!(a.clone(), a.clone())
        }

        #[test]
        fn arb_exp_ok(exp in arb_expression()) {
            prop_assert_eq!(exp.clone(), exp.clone())
        }

        #[test]
        fn expr_round_trip(exp in arb_expression()) {
            let s = exp.clone().to_string();
            let p = _expression(&s);
            prop_assert!(p.is_ok());
            let exp2 = p.unwrap();
            prop_assert_eq!(exp2, exp);
        }
    }
}
