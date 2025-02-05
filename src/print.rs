use crate::expression::{Atom, Define, Expression, Identifier, If, Lambda};
use std::fmt;

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bool(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Self::Float(x) => write!(f, "{x:?}"),
            Self::Int(n) => write!(f, "{n}"),
            Self::Str(s) => write!(f, "{s}"),
            Self::Symbol(s) => write!(f, "{s}"),
        }
    }
}

fn paren<T: Clone + fmt::Display>(xs: &[T], f: &mut fmt::Formatter) -> fmt::Result {
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
        write!(f, "(if {} {} {})", self.cond, self.true_, self.false_)
    }
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(lambda ")?;
        paren(&self.params, f)?;
        write!(f, " #<procedure {:p}>)", &self.body)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Constant(x) => write!(f, "{x}"),
            Self::Define(d) => write!(f, "{d}"),
            Self::If(i) => write!(f, "{i}"),
            Self::Function(g) => write!(f, "#<function {g:p}>"),
            Self::Lambda(l) => write!(f, "{l}"),
            Self::List(xs) => paren(xs, f),
        }
    }
}
