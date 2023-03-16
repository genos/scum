use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Identifier(pub String);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Bool(bool),
    Float(f64),
    Int(i64),
    Str(String),
    Symbol(Identifier),
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

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Constant(Atom),
    List(Vec<Expression>),
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

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Constant(x) => write!(f, "{x}"),
            Expression::List(xs) => _paren(xs, f),
        }
    }
}
