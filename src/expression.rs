use std::{collections::HashMap, sync::Arc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Identifier(Arc<str>);

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum Atom {
    Bool(bool),
    Float(f64),
    Int(i64),
    Str(Arc<str>),
    Symbol(Identifier),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Constant(Atom),
    Define(Arc<Define>),
    If(Arc<If>),
    Function(fn(Arc<[Expression]>) -> Result<Expression, EnvError>),
    Lambda(Arc<Lambda>),
    List(Arc<[Expression]>),
}

#[derive(Debug, Clone)]
pub struct Define {
    pub name: Expression,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct If {
    pub cond: Expression,
    pub true_: Expression,
    pub false_: Expression,
}

#[derive(Debug, Clone)]
pub struct Lambda {
    pub params: Vec<Identifier>,
    pub env: Environment,
    pub body: Expression,
}

macro_rules! expr_from {
    ($($id:ident),*) => {
        $(
        impl From<$id> for Expression {
            fn from(value: $id) -> Self {
                Expression::$id(Arc::new(value))
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

macro_rules! relation {
    ($op:tt) => {
        #[allow(clippy::cast_precision_loss)]
        Expression::Function(|xs: Arc<[Expression]>| {
            let mut result = true;
            for w in xs.windows(2) {
                if let [x, y] = w {
                    match (x, y) {
                        (Expression::Constant(Atom::Bool(a)), Expression::Constant(Atom::Bool(b))) => result = *a $op *b,
                        (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Float(b))) => result = *a $op *b,
                        (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Int(b))) => result = *a $op *b,
                        (Expression::Constant(Atom::Str(a)), Expression::Constant(Atom::Str(b))) => result = *a $op *b,
                        (Expression::Constant(Atom::Symbol(a)), Expression::Constant(Atom::Symbol(b))) => result = a $op b,
                        (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Int(b))) => result = *a $op (*b as f64),
                        (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Float(b))) => result = (*a as f64) $op *b,
                        _ => return Err(EnvError::DifferentConstantTypes(x.clone(), y.clone()))
                    }
                }
            }
            Ok(Expression::Constant(Atom::Bool(result)))
        })
    };
}

macro_rules! binary {
    ($op:tt, $id:expr) => {
        #[allow(clippy::cast_precision_loss)]
        Expression::Function(|xs: Arc<[Expression]>| {
            let mut result = Expression::Constant(Atom::Int($id));
            for w in xs.windows(2) {
                if let [x, y] = w {
                    match (x, y) {
                        (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Float(b))) => {
                            result = Expression::Constant(Atom::Float(*a $op *b));
                        }
                        (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Int(b))) => {
                            result = Expression::Constant(Atom::Int(*a $op *b));
                        }
                        (Expression::Constant(Atom::Float(a)), Expression::Constant(Atom::Int(b))) => {
                            result = Expression::Constant(Atom::Float(*a $op *b as f64));
                        }
                        (Expression::Constant(Atom::Int(a)), Expression::Constant(Atom::Float(b))) => {
                            result = Expression::Constant(Atom::Float((*a as f64) $op *b));
                        }
                        _ => return Err(EnvError::NonNumericArgs(x.clone(), y.clone()))
                    }
                }
            }
            Ok(result)
        })
    };
}

impl Default for Environment {
    // Leave proper handling of float comparisons up to user
    #[allow(clippy::float_cmp)]
    fn default() -> Self {
        Self {
            bindings: HashMap::from([
                (
                    Identifier("println".into()),
                    Expression::Function(|xs: Arc<[Expression]>| {
                        let ys = match xs.len() {
                            1 => xs.first().cloned().expect("len == 1"),
                            _ => Expression::List(xs),
                        };
                        println!("{ys}");
                        Ok(ys)
                    }),
                ),
                (
                    Identifier("list".into()),
                    Expression::Function(|xs: Arc<[Expression]>| Ok(Expression::List(xs))),
                ),
                (Identifier("=".into()), relation!(==)),
                (Identifier("!=".into()), relation!(!=)),
                (Identifier(">".into()), relation!(>)),
                (Identifier("<".into()), relation!(<)),
                (Identifier(">=".into()), relation!(>=)),
                (Identifier("<=".into()), relation!(<=)),
                (Identifier("+".into()), binary!(+, 0)),
                (Identifier("-".into()), binary!(-, 0)),
                (Identifier("*".into()), binary!(*, 1)),
                (Identifier("/".into()), binary!(/, 1)),
            ]),
        }
    }
}

impl Environment {
    pub(crate) fn lookup(&self, identifier: &Identifier) -> Result<Expression, EnvError> {
        self.bindings
            .get(identifier)
            .cloned()
            .ok_or_else(|| EnvError::NotFound(identifier.clone()))
    }

    pub(crate) fn define(&mut self, identifier: Identifier, expression: Expression) {
        self.bindings.insert(identifier, expression);
    }
}
