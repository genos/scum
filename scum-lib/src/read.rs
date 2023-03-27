use crate::expression::{Atom, Expression, Identifier};
use logos::{Lexer, Logos};
use snailquote::unescape;

#[derive(Debug, thiserror::Error)]
pub enum ReadingError {
    #[error("Lexer unexpectedly reached end of input")]
    Eoi,
    #[error("Unexpected closing parenthesis")]
    BadCloseParen,
    #[error("Unexpected token in lexing")]
    LexError,
}

pub(crate) fn read(input: &str) -> Result<Vec<Expression>, ReadingError> {
    let mut lexer = Token::lexer(input);
    read_impl(&mut lexer, true)
}

// https://david-delassus.medium.com/writing-a-simple-lisp-interpreter-in-rust-91dd32ea4d8f
#[derive(Debug, Clone, PartialEq, Logos)]
enum Token {
    #[token("(")]
    OParen,
    #[token(")")]
    CParen,
    //     #[token("define")]
    //     Define,
    //     #[token("lambda")]
    //     Lambda,
    //     #[token("if")]
    //     If,
    #[regex("#t|#f", |lex| lex.slice() == "#t")]
    Bool(bool),
    #[regex(r"[-+]?(0|[1-9]\d*)", |lex| lex.slice().parse(), priority = 3)]
    Int(i64),
    #[regex(r"[+-]?((\d+\.?\d*)|(\.\d+))([eE][+-]?\d+)?", |lex| lex.slice().parse(), priority = 2)]
    Float(f64),
    #[regex(r#""([^"]|\\")*""#, |lex| unescape(lex.slice()))]
    Str(String),
    #[regex(r#"[^\s"\(\)]+"#, |lex| lex.slice().parse())]
    Symbol(String),
    #[error]
    #[regex(r"\s+", logos::skip)]
    Error,
}

fn read_impl(lexer: &mut Lexer<Token>, close_bad: bool) -> Result<Vec<Expression>, ReadingError> {
    let mut expressions = vec![];
    while let Some(token) = lexer.next() {
        match token {
            Token::OParen => expressions.push(Expression::List(read_impl(lexer, false)?)),
            Token::CParen => {
                if close_bad {
                    return Err(ReadingError::BadCloseParen);
                } else {
                    return Ok(expressions);
                }
            }
            Token::Bool(b) => expressions.push(Expression::Constant(Atom::Bool(b))),
            Token::Int(n) => expressions.push(Expression::Constant(Atom::Int(n))),
            Token::Float(f) => expressions.push(Expression::Constant(Atom::Float(f))),
            Token::Str(s) => expressions.push(Expression::Constant(Atom::Str(s))),

            Token::Symbol(s) => expressions.push(Expression::Constant(Atom::Symbol(Identifier(s)))),
            Token::Error => return Err(ReadingError::LexError),
        }
    }
    Ok(expressions)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex_check(input: &str, expected: Token) {
        let mut lexer = Token::lexer(input);
        assert_eq!(lexer.next(), Some(expected));
        assert_eq!(lexer.slice(), input);
    }

    #[test]
    fn lexer() {
        lex_check("(", Token::OParen);
        lex_check(")", Token::CParen);
        // lex_check("define", Token::Define);
        // lex_check("lambda", Token::Lambda);
        // lex_check("if", Token::if);
        lex_check("#t", Token::Bool(true));
        lex_check("#f", Token::Bool(false));
        lex_check("1", Token::Int(1));
        lex_check("+1234", Token::Int(1234));
        lex_check("-1234", Token::Int(-1234));
        lex_check("1.", Token::Float(1.0));
        lex_check("-1.", Token::Float(-1.0));
        lex_check("+1.", Token::Float(1.0));
        lex_check("+1.234", Token::Float(1.234));
        lex_check("-1.234", Token::Float(-1.234));
        lex_check("\"\"", Token::Str(String::new()));
        lex_check("\"asdf\"", Token::Str("asdf".to_string()));
        lex_check("asdf", Token::Symbol("asdf".to_string()));
        lex_check("asdf1234", Token::Symbol("asdf1234".to_string()));
        lex_check("*asdf1234*", Token::Symbol("*asdf1234*".to_string()));
        lex_check("+", Token::Symbol("+".to_string()));
        lex_check("-", Token::Symbol("-".to_string()));
        lex_check("e3", Token::Symbol("e3".to_string()));
    }

    use proptest::prelude::*;

    fn arb_identifier() -> impl Strategy<Value = Identifier> {
        r#"([+[-]*][[:alpha:]!@#$%^&*[-]–—_=+,.<>?]*)|([:alpha:][[:alnum:]!@#$%^&*[-]–—_=+,.<>?]*)"#
            .prop_map(Identifier)
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
            let p = read(&exp.to_string());
            prop_assert!(p.is_ok());
            let exp2 = p.unwrap();
            prop_assert_eq!(exp2.len(), 1);
            prop_assert_eq!(exp2[0].clone(), exp);
        }
    }
}
