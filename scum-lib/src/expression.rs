use im_rc::{hashmap, HashMap};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Identifier(pub Rc<str>);

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Bool(bool),
    Float(f64),
    Int(i64),
    Str(Rc<str>),
    Symbol(Identifier),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Constant(Atom),
    Define(Box<Define>),
    If(Box<If>),
    Function(fn(Vec<Expression>) -> Result<Expression, EnvError>),
    Lambda(Box<Lambda>),
    List(Vec<Expression>),
}

#[derive(Debug, Clone)]
pub struct Define {
    pub name: Expression,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct If {
    pub cond: Expression,
    pub if_true: Expression,
    pub if_false: Expression,
}

#[derive(Debug, Clone)]
pub struct Lambda {
    pub params: Vec<Identifier>,
    pub env: Environment,
    pub body: Expression,
}

macro_rules! expr_from {
    ($($id:ident)*) => {
        $(
        impl From<$id> for Expression {
            fn from(value: $id) -> Self {
                Expression::$id(Box::new(value))
            }
        }
        )*
    };
}

expr_from!(Define If Lambda);

#[derive(Debug, thiserror::Error)]
pub enum EnvError {
    #[error("Expected {expected} arguments, received {actual}")]
    WrongNumberOfArgs { expected: usize, actual: usize },
    #[error("Expected two numeric args, received {0} and {1}")]
    NonNumericArgs(Expression, Expression),
    #[error("Unknown identifier {0}")]
    NotFound(Identifier),
}

#[derive(Debug, Clone)]
pub struct Environment {
    bindings: HashMap<Identifier, Expression>,
}

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
            _ => Err(EnvError::WrongNumberOfArgs{expected: 2, actual: xs.len()}),
        })
    };
}

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
            _ => Err(EnvError::WrongNumberOfArgs{expected: 2, actual: xs.len()}),
        })
    };
}

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
            _ => Err(EnvError::WrongNumberOfArgs{expected: 2, actual: xs.len()}),
        })
    };
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            bindings: hashmap![
                Identifier("=".into()) => equality!(==),
                Identifier("!=".into()) => equality!(!=),
                Identifier(">".into()) => comparison!(>),
                Identifier("<".into()) => comparison!(<),
                Identifier(">=".into()) => comparison!(>=),
                Identifier("<=".into()) => comparison!(<=),
                Identifier("+".into()) => binary_op!(+),
                Identifier("-".into()) => binary_op!(-),
                Identifier("*".into()) => binary_op!(*),
                Identifier("/".into()) => binary_op!(/),
            ],
        }
    }
}

impl Environment {
    pub(crate) fn lookup(&self, identifier: &Identifier) -> Result<Expression, EnvError> {
        self.bindings
            .get(identifier)
            .cloned()
            .ok_or(EnvError::NotFound(identifier.clone()))
    }

    pub(crate) fn define(&mut self, identifier: Identifier, expression: Expression) {
        self.bindings.insert(identifier, expression);
    }
}
