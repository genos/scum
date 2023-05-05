#![deny(unsafe_code)]
mod error;
mod eval;
mod expression;
mod macros;
mod print;
mod read;

pub use error::ScumError;

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
