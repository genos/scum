use crate::expression::{Atom, Expression, Identifier};
use std::fmt;

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Bool(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Atom::Float(x) => write!(f, "{x:?}"),
            Atom::Int(n) => write!(f, "{n}"),
            Atom::Str(s) => write!(f, "\"{s}\""),
            Atom::Symbol(s) => write!(f, "{s}"),
        }
    }
}

fn _paren<T: fmt::Display>(xs: &[T], f: &mut fmt::Formatter) -> fmt::Result {
    let mut sep = "";
    write!(f, "(")?;
    for x in xs {
        write!(f, "{}{}", sep, x)?;
        sep = " ";
    }
    write!(f, ")")
}

impl fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Constant(x) => write!(f, "{x}"),
            Expression::Define(id, x) => write!(f, "(define {id} {x})"),
            Expression::If(cond, x, y) => write!(f, "(if {cond} {x} {y})"),
            Expression::List(xs) => _paren(xs, f),
        }
    }
}
