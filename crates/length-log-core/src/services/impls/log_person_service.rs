use crate::services::PersonService;


#[derive(Debug,Default,Clone)]
pub struct LogPersonService {}

impl PersonService for LogPersonService {
    fn get_by_name(&self, name: &str) -> Result<Option<crate::models::Person>, crate::services::ServiceError> {
        log::info!("searching for name={}", name);
        Ok(None)
    }
    fn get_id_by_name(&self, name: &str) -> Result<String, crate::services::ServiceError> {
        log::info!("searching id for name={}", name);
        Err(format!("not implemnted"))
    }
    fn save(&self, person: crate::models::Person) -> Result<(), crate::services::ServiceError> {
        log::info!("saving person {:?}", person);
        Ok(())
    }
    fn get_all(&self) -> Result<Vec<crate::models::Person>,crate::services::ServiceError> {
        log::info!("listing all persons");
        Ok(Vec::new())
    }
}
