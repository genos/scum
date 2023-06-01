use crate::expression::{Atom, Define, Environment, Expression, Identifier, If, Lambda};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

#[derive(Debug, thiserror::Error)]
pub enum ReadingError {
    #[error("Empty read")]
    Empty,
    #[error("Expected to parse into two pairs, got only one")]
    TooShort,
    #[error("Invalid read: expected one expression, got {0} instead.")]
    Invalid(usize),
    #[error("Expected an expression to parse, but got {0:?} instead.")]
    UnexpectedRule(Rule),
    #[error("Unable to parse float: {0:#?}")]
    ParsingFloat(#[from] std::num::ParseFloatError),
    #[error("Unable to parse int: {0:#?}")]
    ParsingInt(#[from] std::num::ParseIntError),
    #[error("Unexpected compound rule in atomic statement: {0:?}")]
    CompoundInAtom(Rule),
    #[error(
        "Unable to parse {rule} rule: expected {expected_num} expression{plural}, found {found_num}"
    )]
    BadParse {
        rule: String,
        expected_num: usize,
        plural: String,
        found_num: usize,
    },
    #[error("Expected {article} {expected}, but got {expression} instead.")]
    ExpressionMismatch {
        article: String,
        expected: String,
        expression: Expression,
    },
    #[error("Reading error: {location:?}, {line_col:?} {line}")]
    Other {
        location: pest::error::InputLocation,
        line_col: pest::error::LineColLocation,
        line: String,
    },
}

fn bad_parse(
    rule: &str,
    expected_num: usize,
    found_num: usize,
) -> Result<Expression, ReadingError> {
    Err(ReadingError::BadParse {
        rule: rule.to_string(),
        expected_num,
        plural: if expected_num > 1 {
            "s".to_string()
        } else {
            "".to_string()
        },
        found_num,
    })
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

pub(crate) fn read(input: &str, env: &mut Environment) -> Result<Expression, ReadingError> {
    let mut pairs = ScumParser::parse(Rule::input, input)?.collect::<Vec<_>>();
    match pairs.len() {
        0 => Err(ReadingError::Empty),
        1 => Err(ReadingError::TooShort),
        n if n > 2 => Err(ReadingError::Invalid(n - 1)),
        _ => read_impl(pairs.remove(0).into_inner(), env),
    }
}

fn read_impl(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Expression, ReadingError> {
    let mut xs = vec![];
    for x in pairs {
        match x.as_rule() {
            Rule::constant => xs.push(constant_to_expr(x.into_inner())?),
            Rule::define => xs.push(define_to_expr(x.into_inner(), env)?),
            Rule::ifte => xs.push(ifte_to_expr(x.into_inner(), env)?),
            Rule::lambda => xs.push(lambda_to_expr(x.into_inner(), env)?),
            Rule::list => xs.push(list_to_expr(x.into_inner(), env)?),
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
        bad_parse("atom", 1, pieces.len())
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
                pair.as_str().to_string(),
            )))),
            r => Err(ReadingError::CompoundInAtom(r)),
        }
    }
}

fn define_to_expr(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Expression, ReadingError> {
    let mut pieces = pairs.collect::<Vec<_>>();
    if pieces.len() != 2 {
        bad_parse("define", 2, pieces.len())
    } else {
        let name = read_impl(single(pieces.remove(0)), env)?;
        let value = read_impl(single(pieces.remove(0)), env)?;
        Ok(Define { name, value }.into())
    }
}

fn ifte_to_expr(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Expression, ReadingError> {
    let mut pieces = pairs.collect::<Vec<_>>();
    if pieces.len() != 3 {
        bad_parse("if", 3, pieces.len())
    } else {
        let cond = read_impl(single(pieces.remove(0)), env)?;
        let if_true = read_impl(single(pieces.remove(0)), env)?;
        let if_false = read_impl(single(pieces.remove(0)), env)?;
        Ok(If {
            cond,
            if_true,
            if_false,
        }
        .into())
    }
}

fn lambda_to_expr(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Expression, ReadingError> {
    let mut pieces = pairs.collect::<Vec<_>>();
    if pieces.len() != 2 {
        bad_parse("lambda", 2, pieces.len())
    } else {
        let args = read_impl(single(pieces.remove(0)), env)?;
        let mut params = vec![];
        match args {
            Expression::List(xs) => {
                for x in xs {
                    match x {
                        Expression::Constant(Atom::Symbol(i)) => {
                            params.push(i);
                        }
                        _ => {
                            return Err(ReadingError::ExpressionMismatch {
                                article: "a".to_string(),
                                expected: "symbol".to_string(),
                                expression: x,
                            })
                        }
                    }
                }
            }
            _ => {
                return Err(ReadingError::ExpressionMismatch {
                    article: "a".to_string(),
                    expected: "list".to_string(),
                    expression: args.clone(),
                })
            }
        }
        let body = read_impl(single(pieces.remove(0)), env)?;
        Ok(Lambda {
            params,
            env: env.clone(),
            body,
        }
        .into())
    }
}

fn list_to_expr(pairs: Pairs<Rule>, env: &mut Environment) -> Result<Expression, ReadingError> {
    let mut xs = vec![];
    for pair in pairs {
        xs.push(read_impl(single(pair), env)?);
    }
    Ok(Expression::List(xs))
}

fn single(p: Pair<Rule>) -> Pairs<Rule> {
    Pairs::single(p.into_inner().next().unwrap())
}
