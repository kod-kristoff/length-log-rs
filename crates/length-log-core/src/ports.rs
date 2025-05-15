use data::DataService;
pub use error::ServiceError;
use person::PersonService;

use crate::models::Person;

pub trait LengthLogService: PersonService + DataService {
    fn list_persons(&self) -> Result<Vec<Person>, ServiceError>;
}

pub mod data;
mod error;
// pub mod impls;
pub mod person;
