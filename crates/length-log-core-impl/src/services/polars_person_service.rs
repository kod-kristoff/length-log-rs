use std::sync::{Arc,RwLock};

use chrono::NaiveDate;
use length_log_core::{models, services};
use polars::{datatypes::{AnyValue, DataType}, frame::DataFrame, prelude::NamedFrom, series::Series};

pub struct PolarsPersonService {
    persons: RwLock<DataFrame>,
}

impl Default for PolarsPersonService {
    fn default() ->  Self{
        let ids = Series::new_empty("id", &DataType::String);
        let names = Series::new_empty("name", &DataType::String);
        let start_dates = Series::new_empty("start_date", &DataType::Date);
        let persons = RwLock::new(DataFrame::new(vec![ids,names,start_dates]).unwrap());
        Self { persons }
    }
}
impl PolarsPersonService {
  
    pub fn new_shared() -> services::SharedPersonService {
        Arc::new(Self::default())
    }
}


impl services::PersonService for PolarsPersonService {
    fn get_by_name(&self, name: &str) -> Result<Option<models::Person>, services::ServiceError> {
        log::info!("searching for name={}", name);
        Ok(None)
    }
    fn save(&self, models::Person {id,name, start_date}: models::Person) -> Result<(), services::ServiceError> {
        log::info!("saving person id={:?} name={} start_date={:?}", id, name, start_date);
        let ids = Series::new("id", vec![id]);
        let names = Series::new("name", vec![name]);
        let epoch = NaiveDate::from_ymd_opt(1970,1,1).unwrap();
        let start_date = if let Some(start_date) = start_date {
            AnyValue::Date(start_date.signed_duration_since(epoch).num_days() as i32)
        } else {
            AnyValue::Null
        };
        let start_dates = Series::from_any_values("start_date", &[start_date], true).unwrap();
        let person = DataFrame::new(vec![ids,names,start_dates]).unwrap();
        self.persons.write().unwrap().extend(&person).unwrap();
        log::debug!("persons={:?}", self.persons);        
        Ok(())
    }
    fn get_all(&self) -> Result<Vec<models::Person>,services::ServiceError> {
        log::info!("listing all persons");
        Ok(Vec::new())
    }
}