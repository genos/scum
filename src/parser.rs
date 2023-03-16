use crate::expression::{Expression, Identifier};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1, multispace0},
    combinator::{cut, map, map_res, verify},
    error::{context, VerboseError},
    multi::many0,
    number::complete::double,
    sequence::{delimited, pair, preceded},
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

type ParseResult<'a, T = Expression> = IResult<&'a str, T, VerboseError<&'a str>>;

fn _paren<'a, T>(
    inner: impl Parser<&'a str, T, VerboseError<&'a str>>,
) -> impl FnMut(&'a str) -> ParseResult<T> {
    delimited(
        char('('),
        preceded(multispace0, inner),
        cut(preceded(multispace0, char(')'))),
    )
}

fn _is_protected(x: &char, xs: &[char]) -> bool {
    *x == '#' && (xs == vec!['t'] || xs == vec!['f'])
}

fn _identifier(input: &str) -> ParseResult<Identifier> {
    context(
        "id",
        map(
            verify(
                pair(
                    context(
                        "id first",
                        verify(anychar, |c| !(c.is_numeric() || c.is_whitespace())),
                    ),
                    context("id rest", many0(verify(anychar, |c| !c.is_whitespace()))),
                ),
                |(x, xs)| !_is_protected(x, xs),
            ),
            |(x, xs)| Identifier(format!("{}{}", x, xs.iter().collect::<String>())),
        ),
    )(input)
}

fn _expression(input: &str) -> ParseResult {
    context("expr", alt((_atom, _bool, _float, _int, _list)))(input)
}

fn _atom(input: &str) -> ParseResult {
    context("atom", map(_identifier, Expression::Atom))(input)
}

fn _bool(input: &str) -> ParseResult {
    context(
        "bool",
        map(alt((tag("#t"), tag("#f"))), |s| Expression::Bool(s == "#t")),
    )(input)
}

fn _float(input: &str) -> ParseResult {
    context("float", double.map(Expression::Float))(input)
}

fn _int(input: &str) -> ParseResult {
    context(
        "int",
        alt((
            map_res(digit1, |s: &str| s.parse::<i64>().map(Expression::Int)),
            map(preceded(tag("-"), digit1), |s: &str| {
                Expression::Int(-s.parse::<i64>().unwrap())
            }),
        )),
    )(input)
}

fn _list(input: &str) -> ParseResult {
    context("list", _paren(map(many0(_expression), Expression::List)))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    fn arb_identifier() -> impl Strategy<Value = Identifier> {
        (
            any::<char>().prop_filter("first", |c| !(c.is_numeric() || c.is_whitespace())),
            any::<String>().prop_filter("rest", |s| !(s.chars().any(|c| c.is_whitespace()))),
        )
            .prop_map(|(c, s)| Identifier(format!("{c}{s}")))
    }

    fn arb_expression() -> impl Strategy<Value = Expression> {
        // https://docs.rs/proptest/latest/proptest/prelude/trait.Strategy.html#method.prop_recursive
        let leaf = prop_oneof![
            arb_identifier().prop_map(Expression::Atom),
            any::<bool>().prop_map(Expression::Bool),
            any::<i64>().prop_map(Expression::Int),
            any::<f64>().prop_map(Expression::Float),
        ];
        leaf.prop_recursive(
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
        fn expr_round_trip(exp in arb_expression()) {
            let s = exp.clone().to_string();
            let p = _expression(&s);
            prop_assert!(p.is_ok());
            let (rest, exp2) = p.unwrap();
            prop_assert_eq!(rest, "");
            prop_assert_eq!(exp2, exp);
        }
    }
}
