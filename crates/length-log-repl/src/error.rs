use std::{error, fmt, io};

use length_log_core::models::PersonNameError;
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    #[error("Invalid quoting '{0}'")]
    InvalidQuoting(String),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    BadPersonName(#[from] PersonNameError),
    #[error("Unknown '{0}'")]
    Unknown(String),
}
pub type Result<T> = std::result::Result<T, Error>;
