use crate::expression::{Atom, Define, EnvError, Environment, Expression, If, Lambda};
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum EvaluationError {
    #[error("Expected {article} {expected_type}, but evaluation of {input} led to {output}")]
    TypeMismatch {
        article: Arc<str>,
        expected_type: Arc<str>,
        input: Expression,
        output: Expression,
    },
    #[error("Expected {expected} arguments, received {actual}")]
    WrongNumberOfArgs { expected: usize, actual: usize },
    #[error("{0}")]
    EnvError(#[from] EnvError),
}

pub fn eval(
    expression: &Expression,
    environment: &mut Environment,
) -> Result<Expression, EvaluationError> {
    match expression {
        Expression::Constant(Atom::Symbol(s)) => environment.lookup(s).map_err(Into::into),
        Expression::Constant(_) | Expression::Function(_) => Ok(expression.clone()),
        Expression::Define(d) => {
            let Define {
                ref name,
                ref value,
            } = **d;
            let x = if let Expression::Constant(Atom::Symbol(_)) = name {
                name.clone()
            } else {
                eval(name, environment)?
            };
            if let Expression::Constant(Atom::Symbol(i)) = x {
                let y = eval(value, environment)?;
                environment.define(i, y.clone());
                Ok(y)
            } else {
                Err(EvaluationError::TypeMismatch {
                    article: "an".into(),
                    expected_type: "identifier".into(),
                    input: name.clone(),
                    output: x.clone(),
                })
            }
        }
        Expression::If(i) => {
            let If {
                ref cond,
                ref true_,
                ref false_,
            } = **i;
            let cond2 = if let Expression::Constant(Atom::Bool(_)) = cond {
                cond.clone()
            } else {
                eval(cond, environment)?
            };
            if let Expression::Constant(Atom::Bool(b)) = cond2 {
                eval(if b { true_ } else { false_ }, environment)
            } else {
                Err(EvaluationError::TypeMismatch {
                    article: "a".into(),
                    expected_type: "bool".into(),
                    input: cond.clone(),
                    output: cond2,
                })
            }
        }
        Expression::Lambda(l) => Ok(Lambda {
            params: l.params.clone(),
            env: environment.clone(),
            body: l.body.clone(),
        }
        .into()),
        Expression::List(xs) => match xs.first() {
            None => Ok(expression.clone()),
            Some(hd) => match eval(hd, environment)? {
                Expression::Function(f) => xs
                    .iter()
                    .skip(1)
                    .map(|y| eval(y, environment))
                    .collect::<Result<_, _>>()
                    .and_then(|ys| f(ys).map_err(Into::into)),
                Expression::Lambda(l) => {
                    if l.params.len() == xs.len() - 1 {
                        let mut env = l.env.clone();
                        for (p, t) in l.params.iter().zip(xs.iter().skip(1)) {
                            env.define(p.clone(), eval(t, environment)?);
                        }
                        eval(&l.body, &mut env)
                    } else {
                        Err(EvaluationError::WrongNumberOfArgs {
                            expected: l.params.len(),
                            actual: xs.len() - 1,
                        })
                    }
                }
                e => Err(EvaluationError::TypeMismatch {
                    article: "a".into(),
                    expected_type: "function or lambda".into(),
                    input: hd.clone(),
                    output: e,
                }),
            },
        },
    }
}
