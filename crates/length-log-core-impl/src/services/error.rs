use miette::Diagnostic;

#[derive(Debug, Diagnostic, thiserror::Error)]
pub enum PolarsServiceError {
    #[error(transparent)]
    Polars(#[from] polars::error::PolarsError),
}
