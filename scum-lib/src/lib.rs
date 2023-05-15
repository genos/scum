#![deny(unsafe_code)]
mod eval;
mod expression;
mod macros;
mod print;
mod read;

#[derive(Debug, thiserror::Error)]
pub enum ScumError {
    #[error("Reading error: {0}")]
    ReadingError(#[from] crate::read::ReadingError),
    #[error("Env error: {0}")]
    EnvError(#[from] crate::expression::EnvError),
    #[error("Evaluation error: {0}")]
    EvaluationError(#[from] crate::eval::EvaluationError),
}

#[derive(Default)]
pub struct Scum {
    env: crate::expression::Environment,
}

impl Scum {
    pub fn read_eval_string(&mut self, input: &str) -> Result<String, ScumError> {
        crate::read::read(input, &mut self.env)
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
        let one = scum.read_eval_string("(define scale-by (lambda (s) (lambda (x) (* s x))))");
        assert!(one.is_ok());
        let double = scum.read_eval_string("(define double (scale-by 2))");
        assert!(double.is_ok());
        let triple = scum.read_eval_string("(define triple (scale-by 3))");
        assert!(triple.is_ok());
        let double_result = scum.read_eval_string("(double 3)");
        assert!(double_result.is_ok());
        assert_eq!(double_result.unwrap(), "6");
        let triple_result = scum.read_eval_string("(triple 3)");
        assert!(triple_result.is_ok());
        assert_eq!(triple_result.unwrap(), "9");
    }
}
