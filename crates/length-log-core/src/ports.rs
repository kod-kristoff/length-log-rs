/*
   Module `ports` specifies the API by which external modules interact with the length-log domain.

   All traits are bounded by `Send + Sync + 'static`, since their implementations must be shareable
   between request-handling threads.

   Trait methods are explicitly asynchronous, including `Send` bounds on response types,
   since the application is expected to always run in a multithreaded environment.
*/
use chrono::NaiveDate;

pub use error::ServiceError;

use crate::models::datapoint::AddDatapointRequest;
use crate::models::datapoint::AgeRow;
use crate::models::person::AddPersonError;
use crate::models::person::AddPersonRequest;
#[allow(unused_imports)] // PersonName is used in doc comments
use crate::models::person::PersonName;
use crate::models::Person;

/// `LengthLogService` is the public API for the length-log domain.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait LengthLogService {
    /// Add a new [Person].
    ///
    /// # Errors
    ///
    /// - [AddPersonError::Duplicate] if an [Person] with the same [PersonName] already exists.
    fn add_person(&self, req: AddPersonRequest) -> Result<Person, AddPersonError>;
    fn get_person(&self, name: &PersonName) -> Result<Option<Person>, ServiceError>;
    fn add_datapoint(&self, req: AddDatapointRequest) -> Result<(), ServiceError>;
    fn list_person_by_age(&self, name: &PersonName) -> Result<Vec<AgeRow>, ServiceError>;
    fn list_persons(&self) -> Result<Vec<Person>, ServiceError>;
    fn save(&self) -> miette::Result<()>;
}

mod error;

/// `LengthLogRepository` represents a store of blog data.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait LengthLogRepository {
    /// Persist a new [Person].
    ///
    /// # Errors
    ///
    /// - MUST return [AddPersonError::Duplicate] if an [Person] with the same [PersonName]
    ///   already exists.
    fn save_person(&self, person: &Person) -> Result<(), AddPersonError>;
    fn get_person(&self, name: &PersonName) -> Result<Option<Person>, ServiceError>;
    fn save_datapoint(
        &self,
        name: &PersonName,
        date: NaiveDate,
        value: f64,
    ) -> Result<(), ServiceError>;
    fn get_data_with_base(
        &self,
        name: &PersonName,
        start_date: NaiveDate,
    ) -> Result<Vec<AgeRow>, ServiceError>;
    fn dump(&self) -> miette::Result<()>;
}
