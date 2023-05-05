macro_rules! ident {
    ($s:literal) => {
        Identifier($s.to_string())
    };
}

pub(crate) use ident;

macro_rules! equality {
    ($op:tt) => {
        Expression::Function(|xs: Vec<Expression>| match &xs[..] {
            [x, y] => match (x, y) {
                (Expression::Constant(Atom::Bool(a)), Expression::Constant(Atom::Bool(b))) => Ok(Expression::Constant(Atom::Bool(*a $op *b))),
                (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Float(b))) => Ok(Expression::Constant(Atom::Bool(*a $op *b))),
                (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Int(b))) => Ok(Expression::Constant(Atom::Bool(*a $op *b as f64))),
                (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Int(b))) => Ok(Expression::Constant(Atom::Bool(*a $op *b))),
                (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Float(b))) => Ok(Expression::Constant(Atom::Bool(*a as f64 $op *b))),
                (Expression::Constant(Atom::Str(a)), Expression::Constant(Atom::Str(b))) => Ok(Expression::Constant(Atom::Bool(*a $op *b))),
                (Expression::Constant(Atom::Symbol(a)), Expression::Constant(Atom::Symbol(b))) => Ok(Expression::Constant(Atom::Bool(*a $op *b))),
                _ => Ok(Expression::Constant(Atom::Bool(false)))
            },
            _ => Err(EnvError::WrongNumberOfArgs(2, xs.len()).into()),
        })
    };
}

pub(crate) use equality;

macro_rules! comparison {
    ($op:tt) => {
        Expression::Function(|xs: Vec<Expression>| match &xs[..] {
            [x, y] => match (x, y) {
                (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Float(b))) => Ok(Expression::Constant(Atom::Bool(*a $op *b))),
                (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Int(b))) => Ok(Expression::Constant(Atom::Bool(*a $op *b as f64))),
                (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Int(b))) => Ok(Expression::Constant(Atom::Bool(*a $op *b))),
                (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Float(b))) => Ok(Expression::Constant(Atom::Bool((*a as f64) $op *b))),
                _ => Err(EnvError::NonNumericArgs(x.clone(), y.clone()))
            },
            _ => Err(EnvError::WrongNumberOfArgs(2, xs.len()).into()),
        })
    };
}

pub(crate) use comparison;

macro_rules! binary_op {
    ($op:tt) => {
        Expression::Function(|xs: Vec<Expression>| match &xs[..] {
            [x, y] => match (x, y) {
                (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Float(b))) => Ok(Expression::Constant(Atom::Float(*a $op *b))),
                (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Int(b))) => Ok(Expression::Constant(Atom::Float(*a $op *b as f64))),
                (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Int(b))) => Ok(Expression::Constant(Atom::Int(*a $op *b))),
                (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Float(b))) => Ok(Expression::Constant(Atom::Float((*a as f64) $op *b))),
                _ => Err(EnvError::NonNumericArgs(x.clone(), y.clone()))

            }
            _ => Err(EnvError::WrongNumberOfArgs(2, xs.len()).into()),
        })
    };
}

pub(crate) use binary_op;
