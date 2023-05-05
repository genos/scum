use crate::expression::{Atom, EnvError, Environment, Expression};
use std::rc::Rc;

#[derive(Debug, thiserror::Error)]
pub enum EvaluationError {
    #[error("Expected {article} {expected_type}, but evaluation of {input} led to {output}")]
    TypeMismatch {
        article: String,
        expected_type: String,
        input: Expression,
        output: Expression,
    },
    #[error("{0}")]
    EnvError(#[from] EnvError),
}

pub(crate) fn eval(
    expression: &Expression,
    environment: &mut Environment,
) -> Result<Expression, EvaluationError> {
    match expression {
        Expression::Constant(Atom::Symbol(s)) => environment.lookup(s).map_err(Into::into),
        Expression::Constant(_) => Ok(expression.clone()),
        Expression::Define { name, value } => {
            let x = if let Expression::Constant(Atom::Symbol(_)) = **name {
                *name.clone()
            } else {
                eval(name, environment)?
            };
            if let Expression::Constant(Atom::Symbol(i)) = x {
                let y = eval(value, environment)?;
                environment.define(&i, &y);
                Ok(y)
            } else {
                Err(EvaluationError::TypeMismatch {
                    article: "an".to_string(),
                    expected_type: "identifier".to_string(),
                    input: *name.clone(),
                    output: x.clone(),
                })
            }
        }
        Expression::Function(_) => Ok(expression.clone()),
        Expression::If {
            cond,
            if_true,
            if_false,
        } => {
            let cond2 = if let Expression::Constant(Atom::Bool(_)) = **cond {
                *cond.clone()
            } else {
                eval(cond, environment)?
            };
            if let Expression::Constant(Atom::Bool(b)) = cond2 {
                if b {
                    eval(if_true, environment)
                } else {
                    eval(if_false, environment)
                }
            } else {
                Err(EvaluationError::TypeMismatch {
                    article: "a".to_string(),
                    expected_type: "bool".to_string(),
                    input: *cond.clone(),
                    output: cond2,
                })
            }
        }
        Expression::Lambda { params, body, .. } => Ok(Expression::Lambda {
            params: params.clone(),
            env: Environment::new(Some(Rc::new(environment.clone()))).into(),
            body: body.clone(),
        }),
        Expression::List(xs) => {
            if xs.is_empty() {
                Ok(expression.clone())
            } else {
                let (hd, tl) = xs.split_first().expect("List was guaranteed nonempty.");
                match eval(hd, environment)? {
                    Expression::Function(f) => {
                        let mut ys = vec![];
                        for y in tl {
                            ys.push(eval(y, environment)?);
                        }
                        f(ys).map_err(Into::into)
                    }
                    Expression::Lambda { params, env, body } => {
                        if params.len() != tl.len() {
                            Err(EnvError::WrongNumberOfArgs {
                                expected: params.len(),
                                actual: tl.len(),
                            }
                            .into())
                        } else {
                            let mut local = Environment::new(Some(env));
                            for (p, t) in params.iter().zip(tl) {
                                local.define(p, &eval(t, environment)?);
                            }
                            eval(&body, &mut local)
                        }
                    }
                    e => Err(EvaluationError::TypeMismatch {
                        article: "a".to_string(),
                        expected_type: "function".to_string(),
                        input: hd.clone(),
                        output: e,
                    }),
                }
            }
        }
    }
}
