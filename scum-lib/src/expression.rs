use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
    Define(Rc<Define>),
    If(Rc<If>),
    Function(fn(Vec<Expression>) -> Result<Expression, EnvError>),
    Lambda(Rc<Lambda>),
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
    pub env: Rc<Environment>,
    pub body: Expression,
}

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
    bindings: RefCell<HashMap<Identifier, Expression>>,
    outer: Option<Rc<Environment>>,
}

impl Default for Environment {
    fn default() -> Self {
        use crate::macros::{binary_op, comparison, equality, ident};
        Self {
            bindings: RefCell::new(HashMap::from([
                (ident!("="), equality!(==)),
                (ident!("!="), equality!(!=)),
                (ident!(">"), comparison!(>)),
                (ident!("<"), comparison!(<)),
                (ident!(">="), comparison!(>=)),
                (ident!("<="), comparison!(<=)),
                (ident!("+"), binary_op!(+)),
                (ident!("-"), binary_op!(-)),
                (ident!("*"), binary_op!(*)),
                (ident!("/"), binary_op!(/)),
            ])),
            outer: None,
        }
    }
}

impl Environment {
    pub(crate) fn new(outer: Option<Rc<Environment>>) -> Environment {
        Self {
            bindings: Default::default(),
            outer,
        }
    }

    pub(crate) fn lookup(&self, identifier: &Identifier) -> Result<Expression, EnvError> {
        if self.bindings.borrow().contains_key(identifier) {
            Ok(self
                .bindings
                .borrow()
                .get(identifier)
                .cloned()
                .expect("Impossible, we checked it was in there"))
        } else {
            match &self.outer {
                None => Err(EnvError::NotFound(identifier.clone())),
                Some(e) => e.lookup(identifier),
            }
        }
    }

    pub(crate) fn define(&mut self, identifier: &Identifier, expression: &Expression) {
        self.bindings
            .borrow_mut()
            .insert(identifier.clone(), expression.clone());
    }
}
