#[derive(Debug, thiserror::Error)]
pub enum ScumError {
    #[error("Reading error: {0}")]
    ReadingError(#[from] crate::read::ReadingError),
}
