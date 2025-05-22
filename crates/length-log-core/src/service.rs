use std::{fmt, str::FromStr};

use crate::{
    models::{self, AddPersonRequest, Person},
    ports::{LengthLogRepository, LengthLogService, ServiceError},
};

#[derive(Clone)]
pub struct Service<R> {
    repo: R,
}

impl<R> Service<R> {
    pub fn new(repo: R) -> Self {
        log::trace!("creating Service ...");
        Self { repo }
    }
}
impl<R> LengthLogService for Service<R>
where
    R: LengthLogRepository,
{
    fn add_person(&self, req: models::AddPersonRequest) -> Result<Person, models::AddPersonError> {
        let person: Person = req.into();
        log::trace!(
            "adding person '{}' with date = {:?}",
            person.name(),
            person.start_date()
        );

        self.repo.save_person(&person)?;
        Ok(person)
    }

    fn add_datapoint(
        &self,
        req: models::datapoint::AddDatapointRequest,
    ) -> Result<(), ServiceError> {
        todo!()
    }

    fn list_persons(&self) -> Result<Vec<Person>, ServiceError> {
        todo!()
    }
    fn save(&self) -> miette::Result<()> {
        self.repo.dump()?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum AppError {
    BadDate(chrono::ParseError),
    ServiceError(ServiceError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadDate(_) => f.write_str("bad date"),
            Self::ServiceError(_) => f.write_str("ServiceError"),
        }
    }
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        Self::ServiceError(value)
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(value: chrono::ParseError) -> Self {
        Self::BadDate(value)
    }
}
