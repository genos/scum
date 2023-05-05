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
            Atom::Str(s) => write!(f, "{s}"),
            Atom::Symbol(s) => write!(f, "{s}"),
        }
    }
}

fn _paren<T: fmt::Display>(xs: &[T], f: &mut fmt::Formatter) -> fmt::Result {
    let mut sep = "";
    write!(f, "(")?;
    for x in xs {
        write!(f, "{sep}{x}")?;
        sep = " ";
    }
    write!(f, ")")
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Constant(x) => write!(f, "{x}"),
            Expression::Define { name, value } => write!(f, "(define {name} {value})"),
            Expression::If {
                cond,
                if_true,
                if_false,
            } => write!(f, "(if {cond} {if_true} {if_false})"),
            Expression::Function(g) => write!(f, "#<function {g:p}>"),
            Expression::Lambda { params, body, .. } => {
                write!(f, "(lambda ")?;
                _paren(params, f)?;
                write!(f, " {body})")
            }
            Expression::List(xs) => _paren(xs, f),
        }
    }
}
