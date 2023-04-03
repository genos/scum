use crate::expression::{Atom, Expression, FunctionError, Identifier};
use smol_str::SmolStr;
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, thiserror::Error)]
pub enum EvaluationError {
    #[error("Unknown identifier {0}")]
    NotFound(Identifier),
    #[error("Expected identifier, evaluation of {0} led to {1}")]
    ExpectedIdentifier(Expression, Expression),
    #[error("Expected boolean, evaluation of {0} led to {1}")]
    ExpectedBoolean(Expression, Expression),
    #[error("Expected a nonempty list")]
    UnexpectedEmptyList,
    #[error("Expected a function, but evaluation of {0} led to {1}")]
    ExpectedFunction(Expression, Expression),
    #[error("{0}")]
    FunctionError(#[from] crate::expression::FunctionError),
}

#[derive(Debug)]
pub(crate) struct Environment(RefCell<HashMap<Identifier, Expression>>);

impl Default for Environment {
    fn default() -> Self {
        use crate::macros::{binary_op, comparison, equality, ident};
        Self(RefCell::new(HashMap::from([
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
        ])))
    }
}

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
                    Err(EvaluationError::ExpectedIdentifier(*x.clone(), xx.clone()))
                }
            }
            Expression::Function(_) => Ok(expression.clone()),
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
                    Err(EvaluationError::ExpectedBoolean(*cond.clone(), cond2))
                }
            }
            Expression::List(xs) => {
                let (hd, tl) = xs
                    .split_first()
                    .ok_or(EvaluationError::UnexpectedEmptyList)?;
                match self.eval_impl(hd)? {
                    Expression::Function(f) => {
                        let mut ys = vec![];
                        for y in tl {
                            ys.push(self.eval_impl(y)?);
                        }
                        f(ys).map_err(Into::into)
                    }
                    e => Err(EvaluationError::ExpectedFunction(hd.clone(), e)),
                }
            }
        }
    }
}
