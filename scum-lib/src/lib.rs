mod error;
mod expression;
mod parse;

pub use error::ScumError;
pub use expression::Expression;

pub fn parse(input: &str) -> Result<Expression, ScumError> {
    parse::parse(input).map_err(Into::into)
}
