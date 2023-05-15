use crate::expression::{Atom, Define, Expression, Identifier, If, Lambda};
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

impl fmt::Display for Define {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(define {} {})", self.name, self.value)
    }
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(if {} {} {})", self.cond, self.if_true, self.if_false)
    }
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(lambda ")?;
        _paren(&self.params, f)?;
        write!(f, " #<procedure {:p}>)", &self.body)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Constant(x) => write!(f, "{x}"),
            Expression::Define(d) => write!(f, "{d}"),
            Expression::If(i) => write!(f, "{i}"),
            Expression::Function(g) => write!(f, "#<function {g:p}>"),
            Expression::Lambda(l) => write!(f, "{l}"),
            Expression::List(xs) => _paren(xs, f),
        }
    }
}
