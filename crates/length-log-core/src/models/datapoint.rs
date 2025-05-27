use chrono::NaiveDate;

use super::PersonName;

#[derive(Debug, Clone)]
pub struct AddDatapointRequest {
    name: PersonName,
    date: NaiveDate,
    data: f64,
}

impl AddDatapointRequest {
    pub fn new(name: PersonName, date: NaiveDate, data: f64) -> Self {
        Self { name, date, data }
    }

    pub fn data(&self) -> f64 {
        self.data
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn name(&self) -> &PersonName {
        &self.name
    }

    pub fn into_name(self) -> PersonName {
        self.name
    }
}
