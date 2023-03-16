mod error;
mod expression;
mod parser;

pub use error::ScumError;
pub use expression::Expression;

pub fn parse(input: &str) -> Result<Expression, ScumError> {
    parser::parse_impl(input).map_err(Into::into)
}
