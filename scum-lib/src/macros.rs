macro_rules! ident {
    ($s:literal) => {
        crate::expression::Identifier($s.to_string())
    };
}

pub(crate) use ident;

macro_rules! equality {
    ($op:tt) => {
        crate::expression::Expression::Function(|xs: Vec<crate::expression::Expression>| match &xs[..] {
            [x, y] => match (x, y) {
                (crate::expression::Expression::Constant(crate::expression::Atom::Bool(a)), crate::expression::Expression::Constant(crate::expression::Atom::Bool(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Bool(*a $op *b))),
                (crate::expression::Expression::Constant(crate::expression::Atom::Float(a)), crate::expression::Expression::Constant(crate::expression::Atom::Float(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Bool(*a $op *b))),
                (crate::expression::Expression::Constant(crate::expression::Atom::Float(a)), crate::expression::Expression::Constant(crate::expression::Atom::Int(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Bool(*a $op *b as f64))),
                (crate::expression::Expression::Constant(crate::expression::Atom::Int(a)), crate::expression::Expression::Constant(crate::expression::Atom::Int(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Bool(*a $op *b))),
                (crate::expression::Expression::Constant(crate::expression::Atom::Int(a)), crate::expression::Expression::Constant(crate::expression::Atom::Float(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Bool(*a as f64 $op *b))),
                (crate::expression::Expression::Constant(crate::expression::Atom::Str(a)), crate::expression::Expression::Constant(crate::expression::Atom::Str(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Bool(*a $op *b))),
                (crate::expression::Expression::Constant(crate::expression::Atom::Symbol(a)), crate::expression::Expression::Constant(crate::expression::Atom::Symbol(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Bool(*a $op *b))),
                _ => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Bool(false)))
            },
            _ => Err(crate::expression::EnvError::WrongNumberOfArgs{expected: 2, actual: xs.len()}),
        })
    };
}

pub(crate) use equality;

macro_rules! comparison {
    ($op:tt) => {
        crate::expression::Expression::Function(|xs: Vec<crate::expression::Expression>| match &xs[..] {
            [x, y] => match (x, y) {
                (crate::expression::Expression::Constant(crate::expression::Atom::Float(a)), crate::expression::Expression::Constant(crate::expression::Atom::Float(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Bool(*a $op *b))),
                (crate::expression::Expression::Constant(crate::expression::Atom::Float(a)), crate::expression::Expression::Constant(crate::expression::Atom::Int(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Bool(*a $op *b as f64))),
                (crate::expression::Expression::Constant(crate::expression::Atom::Int(a)), crate::expression::Expression::Constant(crate::expression::Atom::Int(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Bool(*a $op *b))),
                (crate::expression::Expression::Constant(crate::expression::Atom::Int(a)), crate::expression::Expression::Constant(crate::expression::Atom::Float(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Bool((*a as f64) $op *b))),
                _ => Err(crate::expression::EnvError::NonNumericArgs(x.clone(), y.clone()))
            },
            _ => Err(crate::expression::EnvError::WrongNumberOfArgs{expected: 2, actual: xs.len()}),
        })
    };
}

pub(crate) use comparison;

macro_rules! binary_op {
    ($op:tt) => {
        crate::expression::Expression::Function(|xs: Vec<crate::expression::Expression>| match &xs[..] {
            [x, y] => match (x, y) {
                (crate::expression::Expression::Constant(crate::expression::Atom::Float(a)), crate::expression::Expression::Constant(crate::expression::Atom::Float(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Float(*a $op *b))),
                (crate::expression::Expression::Constant(crate::expression::Atom::Float(a)), crate::expression::Expression::Constant(crate::expression::Atom::Int(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Float(*a $op *b as f64))),
                (crate::expression::Expression::Constant(crate::expression::Atom::Int(a)), crate::expression::Expression::Constant(crate::expression::Atom::Int(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Int(*a $op *b))),
                (crate::expression::Expression::Constant(crate::expression::Atom::Int(a)), crate::expression::Expression::Constant(crate::expression::Atom::Float(b))) => Ok(crate::expression::Expression::Constant(crate::expression::Atom::Float((*a as f64) $op *b))),
                _ => Err(crate::expression::EnvError::NonNumericArgs(x.clone(), y.clone()))

            }
            _ => Err(crate::expression::EnvError::WrongNumberOfArgs{expected: 2, actual: xs.len()}),
        })
    };
}

pub(crate) use binary_op;
