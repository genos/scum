#[derive(Debug, thiserror::Error)]
pub enum ScumError {
    #[error("Parsing error: {0:#?}")]
    ParsingError(#[from] crate::parse::ParsingError),
}
