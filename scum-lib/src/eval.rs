use crate::expression::{Atom, EnvError, Environment, Expression, Identifier};

#[derive(Debug, thiserror::Error)]
pub enum EvaluationError {
    #[error("Unknown identifier {0}")]
    NotFound(Identifier),
    #[error("Expected {article} {expected_type}, but evaluation of {input} led to {output}")]
    TypeMismatch {
        article: String,
        expected_type: String,
        input: Expression,
        output: Expression,
    },
    #[error("Expected a nonempty list")]
    UnexpectedEmptyList,
    #[error("{0}")]
    EnvError(#[from] EnvError),
}

pub(crate) fn eval(
    expression: &Expression,
    env: &mut Environment,
) -> Result<Expression, EvaluationError> {
    match expression {
        Expression::Constant(Atom::Symbol(s)) => env.lookup(s).map_err(Into::into),
        Expression::Constant(_) => Ok(expression.clone()),
        Expression::Define(x, y) => {
            let xx = if let Expression::Constant(Atom::Symbol(_)) = **x {
                *x.clone()
            } else {
                eval(x, env)?
            };
            if let Expression::Constant(Atom::Symbol(i)) = xx {
                let yy = eval(y, env)?;
                env.define(&i, &yy);
                Ok(yy)
            } else {
                Err(EvaluationError::TypeMismatch {
                    article: "an".to_string(),
                    expected_type: "identifier".to_string(),
                    input: *x.clone(),
                    output: xx.clone(),
                })
            }
        }
        Expression::Function(_) | Expression::Lambda { .. } => Ok(expression.clone()),
        Expression::If(cond, x, y) => {
            let cond2 = if let Expression::Constant(Atom::Bool(_)) = **cond {
                *cond.clone()
            } else {
                eval(cond, env)?
            };
            if let Expression::Constant(Atom::Bool(b)) = cond2 {
                if b {
                    eval(x, env)
                } else {
                    eval(y, env)
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
        Expression::List(xs) => {
            let (hd, tl) = xs
                .split_first()
                .ok_or(EvaluationError::UnexpectedEmptyList)?;
            match eval(hd, env)? {
                Expression::Function(f) => {
                    // apply function to rest of list
                    let mut ys = vec![];
                    for y in tl {
                        ys.push(eval(y, env)?);
                    }
                    f(ys).map_err(Into::into)
                }
                Expression::Lambda { .. } => todo!(),
                // match *args.clone() {
                // // bind args to tail, evaluate, reset, fin.
                // Expression::List(args2) => {
                //     if args2.len() != tl.len() {
                //         Err(FunctionError::WrongNumberOfArgs(args2.len(), tl.len()).into())
                //     } else {
                //         // push arguments as bindings in current environment
                //         let mut old_bindings = HashMap::new();
                //         for (a, x) in args2.iter().zip(tl) {
                //             match a {
                //                 Expression::Constant(Atom::Symbol(i)) => {
                //                     if let Some(y) = self.0.borrow().get(i) {
                //                         old_bindings.insert(i, Some(y.clone()));
                //                     } else {
                //                         old_bindings.insert(i, None);
                //                     }
                //                     self.define(i, x.clone())
                //                 }
                //                 e => {
                //                     return Err(EvaluationError::TypeMismatch {
                //                         article: "an".to_string(),
                //                         expected_type: "identifier".to_string(),
                //                         input: a.clone(),
                //                         output: e.clone(),
                //                     })
                //                 }
                //             }
                //         }
                //         // evaluate in new enviroment
                //         let answer = self.eval(&body)?;
                //         // reset environment to previous state
                //         for (key, val) in old_bindings.iter() {
                //             match val {
                //                 Some(v) => self.define(key, v.clone()),
                //                 None => self.remove(key),
                //             }
                //         }
                //         // fin.
                //         Ok(answer)
                //     }
                // }
                // e => Err(EvaluationError::TypeMismatch {
                //     article: "a".to_string(),
                //     expected_type: "list".to_string(),
                //     input: *args,
                //     output: e,
                // }),
                // },
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
