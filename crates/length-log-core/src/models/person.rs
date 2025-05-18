use std::fmt;

use chrono::{Local, NaiveDate};

#[derive(Debug, Clone)]
pub struct AddPersonRequest {
    name: PersonName,
    start_date: Option<NaiveDate>,
}

impl AddPersonRequest {
    pub fn new(name: PersonName, start_date: Option<NaiveDate>) -> Self {
        Self { name, start_date }
    }

    pub fn name(&self) -> &PersonName {
        &self.name
    }

    pub fn start_date(&self) -> Option<NaiveDate> {
        self.start_date
    }
}

#[derive(Debug, Clone)]
pub struct PersonName(String);

impl fmt::Display for PersonName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl PersonName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, thiserror::Error, miette::Diagnostic)]
pub enum PersonNameError {
    #[error("PersonName can't be empty.")]
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
    start_date: NaiveDate,
}

impl Person {
    pub fn new(name: PersonName, start_date: NaiveDate) -> Self {
        Self { name, start_date }
    }
    pub fn name(&self) -> &PersonName {
        &self.name
    }

    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }
}

#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum AddPersonError {
    #[error("person with {name} already exists")]
    Duplicate { name: PersonName },
    #[error("Unknown error")]
    Unknown(miette::Report),
}

impl From<AddPersonRequest> for Person {
    fn from(value: AddPersonRequest) -> Self {
        Self {
            name: value.name,
            start_date: value
                .start_date
                .unwrap_or_else(|| Local::now().naive_local().date()),
        }
    }
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
