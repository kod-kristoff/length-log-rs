use chrono::NaiveDate;

use super::ServiceError;

pub trait DataService {
    fn save(&self, id: &str, date: NaiveDate, value: f64) -> Result<(), ServiceError>;
}
