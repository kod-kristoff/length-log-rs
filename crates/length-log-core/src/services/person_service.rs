use crate::models::Person;

use super::ServiceError;


pub trait PersonService {
    fn save(&self, person: Person) -> Result<(), ServiceError>;
    fn get_by_name(&self, name: &str) -> Result<Option<Person>, ServiceError>;
    fn get_id_by_name(&self, name: &str) -> Result<String, ServiceError>;
    fn get_all(&self) -> Result<Vec<Person>,ServiceError>;
}
