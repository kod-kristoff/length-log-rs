/*
   Module `ports` specifies the API by which external modules interact with the blog domain.

   All traits are bounded by `Send + Sync + 'static`, since their implementations must be shareable
   between request-handling threads.

   Trait methods are explicitly asynchronous, including `Send` bounds on response types,
   since the application is expected to always run in a multithreaded environment.
*/

use crate::models::person::AddPersonError;
#[allow(unused_imports)] // PersonName is used in doc comments
use crate::models::person::PersonName;
use crate::models::person::{AddPersonRequest, Person};

/// `PersonService` is the public API for the blog domain.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait PersonService {
    /// Add a new [Person].
    ///
    /// # Errors
    ///
    /// - [AddPersonError::Duplicate] if an [Person] with the same [PersonName] already exists.
    fn add_person(&self, req: AddPersonRequest) -> Result<Person, AddPersonError>;
}

/// `PersonRepository` represents a store of blog data.
///
/// External modules must conform to this contract – the domain is not concerned with the
/// implementation details or underlying technology of any external code.
pub trait PersonRepository {
    /// Persist a new [Person].
    ///
    /// # Errors
    ///
    /// - MUST return [AddPersonError::Duplicate] if an [Person] with the same [PersonName]
    ///   already exists.
    fn save(&self, person: &Person) -> Result<(), AddPersonError>;
    fn dump(&self) -> miette::Result<()>;
}

// pub trait PersonService {
//     fn save(&self, person: Person) -> Result<(), ServiceError>;
//     fn get_by_name(&self, name: &str) -> Result<Option<Person>, ServiceError>;
//     fn get_id_by_name(&self, name: &str) -> Result<String, ServiceError>;
//     fn get_all(&self) -> Result<Vec<Person>, ServiceError>;
// }
