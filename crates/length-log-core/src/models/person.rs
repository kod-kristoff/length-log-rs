use std::fmt;

use chrono::{Local, NaiveDate};

#[derive(Debug, Clone)]
pub struct AddPersonRequest {
    name: PersonName,
    startday: Option<NaiveDate>,
}

impl AddPersonRequest {
    pub fn new(name: PersonName, startday: Option<NaiveDate>) -> Self {
        Self { name, startday }
    }
}

#[derive(Debug, Clone)]
pub struct PersonName(String);

impl fmt::Display for PersonName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

#[derive(Debug, Clone)]
pub enum PersonNameError {
    Empty,
}

impl PersonName {
    pub fn new(raw: &str) -> Result<Self, PersonNameError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(PersonNameError::Empty)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}
#[derive(Debug, Clone)]
pub struct Person {
    name: PersonName,
    startday: NaiveDate,
}

impl Person {
    pub fn new(name: PersonName, startday: NaiveDate) -> Self {
        Self { name, startday }
    }
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum AddPersonError {
    #[error("person with {name} already exists")]
    Duplicate { name: PersonName },
    #[error("Unknown error")]
    Unknown(miette::Report),
}
// #[derive(Debug, Default, Clone)]
// pub struct Person {
//     pub name: PersonName,
//     pub start_date: NaiveDate,
// }
//
// impl Person {
//     pub fn new(name: PersonName, start_date: Option<NaiveDate>) -> Self {
//         let start_date = start_date.unwrap_or_else(|| Local::now().naive_local().date());
//         Self {
//             id,
//             name,
//             start_date,
//         }
//     }
//
//     pub fn with_name(name: PersonName) -> Self {
//         Self::with_name_and_start_date(name, None)
//     }
//     pub fn with_name_and_start_date(name: PersonName, start_date: Option<NaiveDate>) -> Self {
//         let id = Ulid::new().to_string();
//         Self::new(id, name, start_date)
//     }
// }
