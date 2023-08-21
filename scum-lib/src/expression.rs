use im_rc::{hashmap, HashMap, Vector};
use itertools::Itertools;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Identifier(Rc<str>);

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl Identifier {
    pub fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Debug, Clone)]
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
    Define(Rc<Define>),
    If(Rc<If>),
    Function(fn(Rc<[Expression]>) -> Result<Expression, EnvError>),
    Lambda(Rc<Lambda>),
    List(Vector<Expression>),
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
    pub params: Vector<Identifier>,
    pub env: Environment,
    pub body: Expression,
}

macro_rules! expr_from {
    ($($id:ident),*) => {
        $(
        impl From<$id> for Expression {
            fn from(value: $id) -> Self {
                Expression::$id(Rc::new(value))
            }
        }
        )*
    };
}

expr_from!(Define, If, Lambda);

#[derive(Debug, thiserror::Error)]
pub enum EnvError {
    #[error("Different constant types; received {0} and {1}")]
    DifferentConstantTypes(Expression, Expression),
    #[error("Expected two numeric args; received {0} and {1}")]
    NonNumericArgs(Expression, Expression),
    #[error("Unknown identifier {0}")]
    NotFound(Identifier),
}

#[derive(Debug, Clone)]
pub struct Environment {
    bindings: HashMap<Identifier, Expression>,
}

macro_rules! apply_rel {
    ($op:tt, $x:expr, $y:expr) => {
        match ($x, $y) {
            (Expression::Constant(Atom::Bool(a)), Expression::Constant(Atom::Bool(b))) => *a $op *b,
            (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Float(b))) => *a $op *b,
            (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Int(b))) => *a $op *b,
            (Expression::Constant(Atom::Str(a)), Expression::Constant(Atom::Str(b))) => *a $op *b,
            (Expression::Constant(Atom::Symbol(a)), Expression::Constant(Atom::Symbol(b))) => *a $op *b,
            (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Int(b))) => *a $op (*b as f64),
            (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Float(b))) => (*a as f64) $op *b,
            (_, _) => return Err(EnvError::DifferentConstantTypes($x.clone(), $y.clone()))
        }
    };
}

macro_rules! apply_op {
    ($op:tt, $x:expr, $y:expr) => {
        match ($x, $y) {
            (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Float(b))) => Expression::Constant(Atom::Float(*a $op *b)),
            (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Int(b))) => Expression::Constant(Atom::Int(*a $op *b)),
            (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Int(b))) => Expression::Constant(Atom::Float(*a $op *b as f64)),
            (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Float(b))) => Expression::Constant(Atom::Float((*a as f64) $op *b)),
            _ => return Err(EnvError::NonNumericArgs($x.clone(), $y.clone()))
        }
    };
}

macro_rules! relation {
    ($op:tt) => {
        Expression::Function(|xs: Rc<[Expression]>| {
            let z = true;
            if xs.is_empty() {
                Ok(Expression::Constant(Atom::Bool(z)))
            } else {
                let mut result = z;
                for (x, y) in xs.iter().tuple_windows() {
                    result = apply_rel!($op, x, y);
                }
                Ok(Expression::Constant(Atom::Bool(result)))
            }
        })
    };
}

macro_rules! binary {
    ($op:tt, $zero:expr) => {
        Expression::Function(|xs: Rc<[Expression]>| {
            let z = Expression::Constant(Atom::Int($zero));
            if xs.is_empty() {
                Ok(z)
            } else {
                let mut result = z;
                for (x, y) in xs.iter().tuple_windows() {
                    result = apply_op!($op, x, y);
                }
                Ok(result)
            }
        })
    };
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            bindings: hashmap![
                Identifier("=".into()) => relation!(==),
                Identifier("!=".into()) => relation!(!=),
                Identifier(">".into()) => relation!(>),
                Identifier("<".into()) => relation!(<),
                Identifier(">=".into()) => relation!(>=),
                Identifier("<=".into()) => relation!(<=),
                Identifier("+".into()) => binary!(+, 0),
                Identifier("-".into()) => binary!(-, 0),
                Identifier("*".into()) => binary!(*, 1),
                Identifier("/".into()) => binary!(/, 1),
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
