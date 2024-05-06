//! It's about time I made a Lisp of some sort…
#![forbid(missing_docs)]
#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
mod eval;
mod expression;
mod print;
mod read;

#[derive(Debug, thiserror::Error)]
/// Top-level errors
pub enum ScumError {
    /// Reading error: {`0`}
    #[error("Reading error: {0}")]
    ReadingError(#[from] crate::read::ReadingError),
    /// Env error: {`0`}
    #[error("Env error: {0}")]
    EnvError(#[from] crate::expression::EnvError),
    /// Evaluation error: {`0`}
    #[error("Evaluation error: {0}")]
    EvaluationError(#[from] crate::eval::EvaluationError),
}

/// A Scum top-level consists of an environment…
#[derive(Default)]
pub struct Scum {
    env: crate::expression::Environment,
}

impl Scum {
    /// …and a REPL.
    ///
    /// # Errors
    /// If evaluating the expression in the current environment returns an error, that will be
    /// bubbled up to the top-level.
    pub fn read_eval_print(&mut self, input: &str) -> Result<String, ScumError> {
        crate::read::read(input, &self.env)
            .map_err(ScumError::from)
            .and_then(|x| crate::eval::eval(&x, &mut self.env).map_err(ScumError::from))
            .map(|expression| expression.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn nested_lambda() {
        let mut scum = Scum::default();
        let one = scum.read_eval_print("(define scale-by (lambda (s) (lambda (x) (* s x))))");
        assert!(one.is_ok());
        let double = scum.read_eval_print("(define double (scale-by 2))");
        assert!(double.is_ok());
        let triple = scum.read_eval_print("(define triple (scale-by 3))");
        assert!(triple.is_ok());
        let double_result = scum.read_eval_print("(double 3)");
        assert!(double_result.is_ok());
        assert_eq!(double_result.unwrap(), "6");
        let triple_result = scum.read_eval_print("(triple 3)");
        assert!(triple_result.is_ok());
        assert_eq!(triple_result.unwrap(), "9");
    }
}
