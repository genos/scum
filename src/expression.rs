use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Identifier(pub String);

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Atom(Identifier),
    Bool(bool),
    Int(i64),
    Float(f64),
    List(Vec<Expression>),
}

fn _paren<T: fmt::Display>(xs: &[T], f: &mut fmt::Formatter) -> fmt::Result {
    let mut sep = "";
    write!(f, "(")?;
    for x in xs {
        write!(f, "{}{}", x, sep)?;
        sep = " ";
    }
    write!(f, ")")
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Atom(i) => write!(f, "{i}"),
            Expression::Bool(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Expression::Int(n) => write!(f, "{n}"),
            Expression::Float(x) => write!(f, "{x}"),
            Expression::List(xs) => _paren(xs, f),
        }
    }
}
