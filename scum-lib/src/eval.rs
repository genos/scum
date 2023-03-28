use crate::expression::{Atom, Expression, Identifier};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, thiserror::Error)]
pub enum EvaluationError {
    #[error("Unknown identifier {0}")]
    NotFound(Identifier),
    #[error("Expected identifier, evaluation of {0} led to {1}")]
    ExpectedIdentifier(String, String),
    #[error("Expected boolean, evaluation of {0} led to {1}")]
    ExpectedBoolean(String, String),
}

#[derive(Default, Debug)]
pub(crate) struct Environment(RefCell<HashMap<Identifier, Rc<Expression>>>);

impl Environment {
    pub(crate) fn eval(&self, expression: Expression) -> Result<Rc<Expression>, EvaluationError> {
        self.eval_impl(Rc::new(expression))
    }
    fn define(&self, identifier: &Identifier, expr: Rc<Expression>) {
        self.0.borrow_mut().insert(identifier.clone(), expr);
    }
    fn lookup(&self, identifier: &Identifier) -> Result<Rc<Expression>, EvaluationError> {
        self.0
            .borrow()
            .get(identifier)
            .ok_or_else(|| EvaluationError::NotFound(identifier.clone()))
            .cloned()
    }
    fn eval_impl(&self, expression: Rc<Expression>) -> Result<Rc<Expression>, EvaluationError> {
        match &*expression {
            Expression::Constant(Atom::Symbol(s)) => self.lookup(s),
            Expression::Constant(_) => Ok(expression.clone()),
            Expression::Define(x, y) => {
                let xx = if let Expression::Constant(Atom::Symbol(_)) = **x {
                    x.clone()
                } else {
                    self.eval_impl(x.clone())?
                };
                if let Expression::Constant(Atom::Symbol(ref i)) = *xx {
                    let yy = self.eval_impl(y.clone())?;
                    self.define(i, yy.clone());
                    Ok(yy)
                } else {
                    Err(EvaluationError::ExpectedIdentifier(
                        x.clone().to_string(),
                        xx.clone().to_string(),
                    ))
                }
            }
            Expression::If(cond, x, y) => {
                let cond2 = if let Expression::Constant(Atom::Bool(_)) = **cond {
                    cond.clone()
                } else {
                    self.eval_impl(cond.clone())?
                };
                if let Expression::Constant(Atom::Bool(ref b)) = *cond2 {
                    if *b {
                        self.eval_impl(x.clone())
                    } else {
                        self.eval_impl(y.clone())
                    }
                } else {
                    Err(EvaluationError::ExpectedBoolean(
                        cond.to_string(),
                        cond2.to_string(),
                    ))
                }
            }
            Expression::List(xs) => {
                let mut ys = vec![];
                for x in xs {
                    let y = self.eval_impl(x.clone())?;
                    ys.push(y);
                }
                Ok(Rc::new(Expression::List(ys)))
            }
        }
    }
}
