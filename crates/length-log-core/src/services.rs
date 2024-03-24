mod person_service;
mod data_service;
mod error;
pub mod impls;

use std::sync::Arc;

pub use error::ServiceError;
pub use person_service::PersonService;
pub use data_service::DataService;

pub type SharedPersonService = Arc<dyn PersonService>;
pub type SharedDataService = Arc<dyn DataService>;