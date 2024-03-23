use chrono::NaiveDate;

#[derive(Debug,Default,Clone)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub start_date: Option<NaiveDate>,
}

