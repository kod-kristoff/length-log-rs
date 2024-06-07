use chrono::NaiveDate;

#[derive(Debug, Default, Clone)]
pub struct DataPoint {
    id: String,
    date: NaiveDate,
    value: f64,
}

impl DataPoint {
    pub fn new(id: String, date: NaiveDate, value: f64) -> Self {
        Self { id, date, value }
    }
}
