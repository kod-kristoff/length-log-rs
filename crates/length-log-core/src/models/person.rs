use chrono::{Local, NaiveDate};
use ulid::Ulid;
#[derive(Debug,Default,Clone)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub start_date: NaiveDate,
}

impl Person {
    pub fn new(id: String,name: String, start_date: Option<NaiveDate>) -> Self {
        let start_date = start_date.unwrap_or_else(|| Local::now().naive_local().date());
        Self { id, name, start_date }
    }

    pub fn with_name(name: String) -> Self {
        Self::with_name_and_start_date(name, None)
    }
    pub fn with_name_and_start_date(name: String, start_date: Option<NaiveDate>) -> Self {
        let id = Ulid::new().to_string();
        Self::new(id,name,start_date)
    }
}

