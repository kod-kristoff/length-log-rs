use chrono::NaiveDate;

use super::PersonName;

pub struct AddDatapointRequest {
    name: PersonName,
    date: Option<NaiveDate>,
    data: f64,
}

impl AddDatapointRequest {
    pub fn new(name: PersonName, date: Option<NaiveDate>, data: f64) -> Self {
        Self { name, date, data }
    }
}
