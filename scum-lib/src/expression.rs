use im_rc::{hashmap, HashMap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Bool(bool),
    Float(f64),
    Int(i64),
    Str(String),
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

macro_rules! expr_from {
    ($id:ident) => {
        impl From<$id> for Expression {
            fn from(value: $id) -> Self {
                Expression::$id(Box::new(value))
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct Define {
    pub name: Expression,
    pub value: Expression,
}

expr_from!(Define);

#[derive(Debug, Clone)]
pub struct If {
    pub cond: Expression,
    pub if_true: Expression,
    pub if_false: Expression,
}

expr_from!(If);

#[derive(Debug, Clone)]
pub struct Lambda {
    pub params: Vec<Identifier>,
    pub env: Environment,
    pub body: Expression,
}

expr_from!(Lambda);

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
                Identifier("=".to_string()) => equality!(==),
                Identifier("!=".to_string()) => equality!(!=),
                Identifier(">".to_string()) => comparison!(>),
                Identifier("<".to_string()) => comparison!(<),
                Identifier(">=".to_string()) => comparison!(>=),
                Identifier("<=".to_string()) => comparison!(<=),
                Identifier("+".to_string()) => binary_op!(+),
                Identifier("-".to_string()) => binary_op!(-),
                Identifier("*".to_string()) => binary_op!(*),
                Identifier("/".to_string()) => binary_op!(/),
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

    pub(crate) fn define(&mut self, identifier: &Identifier, expression: &Expression) {
        self.bindings.insert(identifier.clone(), expression.clone());
    }
}
