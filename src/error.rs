#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("Unknown error: {0}")]
    Unknown(String),
}
pub type Result<T> = std::result::Result<T, Error>;
