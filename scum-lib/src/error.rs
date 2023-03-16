use crate::parser::Rule;

#[derive(Debug, thiserror::Error)]
pub enum ScumError {
    #[error("Unable to parse float: {0:#?}")]
    ParsingFloat(#[from] std::num::ParseFloatError),
    #[error("Unable to parse int: {0:#?}")]
    ParsingInt(#[from] std::num::ParseIntError),
    #[error("Parsing error: {0:#?}")]
    GeneralParsing(#[from] pest::error::Error<Rule>),
}

pub type ScumResult<T> = Result<T, ScumError>;
