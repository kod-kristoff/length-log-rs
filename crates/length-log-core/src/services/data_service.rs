use chrono::NaiveDate;

use crate::models::DataPoint;

use super::ServiceError;

pub trait DataService {
    fn save(&self, id: &str, date: NaiveDate, value: f64) -> Result<(), ServiceError>;
    fn get_all(&self) -> Result<Vec<DataPoint>, ServiceError>;
}
