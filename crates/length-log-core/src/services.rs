mod data_service;
mod error;
pub mod impls;
mod person_service;

use std::sync::Arc;

pub use data_service::DataService;
pub use error::ServiceError;
pub use person_service::PersonService;

pub type SharedPersonService = Arc<dyn PersonService>;
pub type SharedDataService = Arc<dyn DataService>;
