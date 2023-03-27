mod error;
mod expression;
mod read;

pub use error::ScumError;
pub use expression::Expression;

pub fn read(input: &str) -> Result<Vec<Expression>, ScumError> {
    crate::read::read(input).map_err(Into::into)
}
