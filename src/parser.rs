use crate::expression::{Atom, Expression, Identifier};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1, multispace0, multispace1},
    combinator::{cut, fail, map, map_res, value, verify},
    error::{context, VerboseError},
    multi::{many0, separated_list0},
    number::complete::double,
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};

fn _conv_err(e: VerboseError<&str>) -> VerboseError<String> {
    VerboseError {
        errors: e
            .errors
            .into_iter()
            .map(|(s, k)| (s.to_string(), k))
            .collect(),
    }
}

fn _is_protected(x: &char, xs: &[char]) -> bool {
    x.is_whitespace()
        || x.is_numeric()
        || *x == '('
        || *x == ')'
        || (*x == '#' && xs.first().map(|&c| c == 't' || c == 'f').unwrap_or(false))
        || ((*x == '-' || *x == '+') && xs.first().map(|&c| c.is_numeric()).unwrap_or(false))
        || xs
            .iter()
            .any(|&c| c == '(' || c == ')' || c.is_whitespace())
}

type ParseResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

fn _identifier(input: &str) -> ParseResult<Identifier> {
    map(
        verify(pair(anychar, many0(anychar)), |(x, xs)| {
            !_is_protected(x, xs)
        }),
        |(x, xs)| Identifier(format!("{}{}", x, xs.iter().collect::<String>())),
    )(input)
}

fn _bool(input: &str) -> ParseResult<Atom> {
    alt((
        value(Atom::Bool(true), tag("#t")),
        value(Atom::Bool(false), tag("#f")),
    ))(input)
}

fn _float(input: &str) -> ParseResult<Atom> {
    map(double, Atom::Float)(input)
}

fn _int(input: &str) -> ParseResult<Atom> {
    alt((
        map_res(digit1, |s: &str| s.parse::<i64>().map(Atom::Int)),
        map(preceded(tag("-"), digit1), |s: &str| {
            Atom::Int(-s.parse::<i64>().unwrap())
        }),
    ))(input)
    .and_then(|(i, a)| {
        if i.starts_with('.') || i.starts_with('e') {
            fail(input)
        } else {
            Ok((i, a))
        }
    })
}

fn _symbol(input: &str) -> ParseResult<Atom> {
    map(_identifier, Atom::Symbol)(input)
}

fn _atom(input: &str) -> ParseResult<Atom> {
    // Be sure to have _int before _float!
    alt((_bool, _int, _float, _symbol))(input)
}

fn _paren<'a, T>(
    inner: impl Parser<&'a str, T, VerboseError<&'a str>>,
) -> impl FnMut(&'a str) -> ParseResult<T> {
    delimited(
        terminated(char('('), multispace0),
        inner,
        cut(preceded(multispace0, char(')'))),
    )
}

fn _list(input: &str) -> ParseResult<Expression> {
    _paren(map(
        separated_list0(multispace1, _expression),
        Expression::List,
    ))(input)
}

fn _expression(input: &str) -> ParseResult<Expression> {
    alt((map(_atom, Expression::Constant), _list))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    fn arb_identifier() -> impl Strategy<Value = Identifier> {
        (any::<char>(), any::<Vec<char>>())
            .prop_filter("filters", |(x, xs)| !_is_protected(x, xs))
            .prop_map(|(c, s)| Identifier(format!("{}{}", c, s.iter().collect::<String>())))
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
        fn id_round_trip(i in arb_identifier()) {
            let s = i.clone().to_string();
            let p = _identifier(&s);
            prop_assert!(p.is_ok());
            let (rest, i2) = p.unwrap();
            prop_assert_eq!(rest, "");
            prop_assert_eq!(i2, i);
        }

        #[test]
        fn atom_round_trip(a in arb_atom()) {
            let s = a.clone().to_string();
            dbg!(&s);
            let p = _atom(&s);
            prop_assert!(p.is_ok());
            let (rest, a2) = p.unwrap();
            prop_assert_eq!(rest, "");
            prop_assert_eq!(a2, a);
        }


        #[test]
        fn expr_round_trip(exp in arb_expression()) {
            // dbg!(&exp);
            let s = exp.clone().to_string();
            let p = _expression(&s);
            dbg!(&s);
            dbg!(&p);
            prop_assert!(p.is_ok());
            let (rest, exp2) = p.unwrap();
            prop_assert_eq!(rest, "");
            prop_assert_eq!(exp2, exp);
        }
    }
}
