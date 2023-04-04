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
    env: crate::eval::Environment,
}

impl Scum {
    pub fn read_eval_string(&self, input: &str) -> Result<String, ScumError> {
        crate::read::read(input)
            .map_err(ScumError::from)
            .and_then(|x| self.env.eval(&x).map_err(ScumError::from))
            .map(|expression| expression.to_string())
    }
}
