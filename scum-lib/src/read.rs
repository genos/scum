use crate::expression::{Atom, Expression, Identifier};
use once_cell::sync::Lazy;
use regex::{CaptureMatches, Regex};
use std::iter::Peekable;

#[derive(Debug, thiserror::Error)]
pub enum ReadingError {
    #[error("Unexpeted end of input")]
    Eoi,
    #[error("Unclosed string")]
    UnclosedString,
    #[error("Unrecognized token: {0}")]
    UnrecognizedToken(String),
}
static TOKEN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r###"\s*([()]|"(?:\\.|[^\\"])*"?|;.*|[^\s(";)]+)"###).unwrap());

pub(crate) fn read(input: &str) -> Result<Expression, ReadingError> {
    let mut reader: Peekable<CaptureMatches> = new_reader(input);
    todo!()
}

fn new_reader(input: &str) -> Peekable<CaptureMatches> {
    TOKEN.captures_iter(input).peekable()
}

static INT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^-?[0-9]+$").unwrap());
static FLOAT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[-+]?[0-9]*\.?[0-9]+(?:[eE][-+]?[0-9]+)?").unwrap());
static STR: Lazy<Regex> = Lazy::new(|| Regex::new(r#""([^"]|\\")*""#).unwrap());
static SYMBOL: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[^\s"\(\)]+"#).unwrap());

fn read_atom(reader: &mut Peekable<CaptureMatches>) -> Result<Atom, ReadingError> {
    match reader.next() {
        None => Err(ReadingError::Eoi),
        Some(capture) => match &capture[1] {
            "#t" => Ok(Atom::Bool(true)),
            "#f" => Ok(Atom::Bool(false)),
            t if INT.is_match(t) => Ok(Atom::Int(t.parse().unwrap())),
            t if FLOAT.is_match(t) && !t.starts_with('e') && !t.starts_with('E') => {
                Ok(Atom::Float(t.parse().unwrap()))
            }
            t if STR.is_match(t) => Ok(Atom::Str(t[1..t.len() - 1].to_string())),
            t if t.starts_with('"') => Err(ReadingError::UnclosedString), // TODO necessary?
            t if SYMBOL.is_match(t) => Ok(Atom::Symbol(Identifier(t.to_string()))),
            t => Err(ReadingError::UnrecognizedToken(t.to_string())),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::read_true("#t", Atom::Bool(true))]
    #[case::read_false("#f", Atom::Bool(false))]
    #[case::read_int_1("1", Atom::Int(1))]
    #[case::read_int_neg_1("-1", Atom::Int(-1))]
    #[case::read_float_1("1.", Atom::Float(1.0))]
    #[case::read_float_neg_1("-1.", Atom::Float(-1.0))]
    #[case::read_float_point_1(".1", Atom::Float(0.1))]
    #[case::read_float_neg_point_1("-.1", Atom::Float(-0.1))]
    #[case::read_float_1e3("1e3", Atom::Float(1e3))]
    #[case::read_float_neg_1e3("-1e3", Atom::Float(-1e3))]
    #[case::read_float_1ep3("1e+3", Atom::Float(1e3))]
    #[case::read_float_neg_1ep3("-1e+3", Atom::Float(-1e3))]
    #[case::read_float_1en3("1e-3", Atom::Float(1e-3))]
    #[case::read_float_neg_1en3("-1e-3", Atom::Float(-1e-3))]
    #[case::read_string_empty("\"\"", Atom::Str(String::new()))]
    #[case::read_string_asdf("\"asdf\"", Atom::Str("asdf".to_string()))]
    #[case::read_symbol_asdf("asdf", Atom::Symbol(Identifier("asdf".to_string())))]
    #[case::read_symbol_plus_asdf_plus("+asdf+", Atom::Symbol(Identifier("+asdf+".to_string())))]
    #[case::read_symbol_plus("+", Atom::Symbol(Identifier("+".to_string())))]
    #[case::read_symbol_e3("e3", Atom::Symbol(Identifier("e3".to_string())))]
    fn reading_atoms(#[case] input: &str, #[case] expected: Atom) {
        let a = read_atom(&mut new_reader(input));
        assert!(a.is_ok());
        assert_eq!(a.unwrap(), expected);
    }
}
