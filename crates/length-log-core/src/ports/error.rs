use chrono::NaiveDate;
use miette::Diagnostic;

use crate::models::PersonName;

#[derive(Debug, Diagnostic, thiserror::Error)]
pub enum ServiceError {
    #[error("No person with name={0}")]
    PersonNotFound(PersonName),
    #[error("Date '{date}' is earlier than start date '{start_date}'")]
    DateEarlierThanStart {
        date: NaiveDate,
        start_date: NaiveDate,
    },
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("Custom Error")]
    #[diagnostic(transparent)]
    Custom(Box<dyn Diagnostic>),
}
