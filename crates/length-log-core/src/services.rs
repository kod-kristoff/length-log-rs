mod person_service;
mod error;
pub mod impls;

use std::sync::Arc;

pub use error::ServiceError;
pub use person_service::PersonService;

pub type SharedPersonService = Arc<dyn PersonService>;
