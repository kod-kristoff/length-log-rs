
use miette::Diagnostic;

#[derive(Debug, Diagnostic, thiserror::Error)]
pub enum ServiceError {
    #[error("No person with name={0}")]
    PersonNotFound(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("Custom Error")]
    #[diagnostic(transparent)]
    Custom(Box<dyn Diagnostic>),
}
