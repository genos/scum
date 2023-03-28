mod error;
mod eval;
mod expression;
mod print;
mod read;

pub use error::ScumError;

#[derive(Default)]
pub struct Scum {
    //env: crate::eval::Environment,
}

impl Scum {
    pub fn read_eval_string(&self, input: &str) -> Result<String, ScumError> {
        let mut output = String::new();
        // for expression in crate::read::read(input).map_err(ScumError::from)? {
        //     output = self.env.eval(expression)?.to_string();
        // }
        Ok(output)
    }
}
