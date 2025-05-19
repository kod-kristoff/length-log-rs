use chrono::NaiveDate;

use crate::models::{datapoint::AddDatapointRequest, PersonName};

use super::ServiceError;

pub trait DataService {
    fn add_datapoint(&self, req: AddDatapointRequest) -> Result<(), ServiceError>;
}
pub trait DataRepository {
    fn save(&self, name: &PersonName, date: NaiveDate, value: f64) -> Result<(), ServiceError>;
    fn dump(&self) -> miette::Result<()>;
}
