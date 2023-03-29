use crate::expression::{Atom, Expression, Identifier};
use std::{cell::RefCell, collections::HashMap};

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
pub(crate) struct Environment(RefCell<HashMap<Identifier, Expression>>);

// impl Default for Environment {
//     fn default() -> Self {
//         Self(RefCell::new(HashMap::from([
//             (ident!("+"), todo!()),
//             (ident!("-"), todo!()),
//             (ident!("*"), todo!()),
//             (ident!("/"), todo!()),
//         ])))
//     }
// }

impl Environment {
    pub(crate) fn eval(&self, expression: Expression) -> Result<Expression, EvaluationError> {
        self.eval_impl(&expression)
    }
    fn define(&self, identifier: &Identifier, expr: Expression) {
        self.0.borrow_mut().insert(identifier.clone(), expr);
    }
    fn lookup(&self, identifier: &Identifier) -> Result<Expression, EvaluationError> {
        self.0
            .borrow()
            .get(identifier)
            .ok_or_else(|| EvaluationError::NotFound(identifier.clone()))
            .cloned()
    }
    fn eval_impl(&self, expression: &Expression) -> Result<Expression, EvaluationError> {
        match expression {
            Expression::Constant(Atom::Symbol(ref s)) => self.lookup(s),
            Expression::Constant(_) => Ok(expression.clone()),
            Expression::Define(ref x, ref y) => {
                let xx = if let Expression::Constant(Atom::Symbol(_)) = **x {
                    *x.clone()
                } else {
                    self.eval_impl(x)?
                };
                if let Expression::Constant(Atom::Symbol(i)) = xx {
                    let yy = self.eval_impl(y)?;
                    self.define(&i, yy.clone());
                    Ok(yy)
                } else {
                    Err(EvaluationError::ExpectedIdentifier(
                        x.to_string(),
                        xx.to_string(),
                    ))
                }
            }
            Expression::If(ref cond, ref x, ref y) => {
                let cond2 = if let Expression::Constant(Atom::Bool(_)) = **cond {
                    *cond.clone()
                } else {
                    self.eval_impl(cond)?
                };
                if let Expression::Constant(Atom::Bool(b)) = cond2 {
                    if b {
                        self.eval_impl(x)
                    } else {
                        self.eval_impl(y)
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
                    let y = self.eval_impl(x)?;
                    ys.push(y);
                }
                Ok(Expression::List(ys))
            }
        }
    }
}
