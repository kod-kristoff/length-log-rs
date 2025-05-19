use std::{fmt, str::FromStr};

use chrono::{Local, NaiveDate};
// use crate::error::Error;
// use nom::{bytes::complete::tag, combinator::map, IResult};
// use polars::prelude::DataFrame;

// fn person(i: &str) -> IResult<&str, &str> {
//     tag("person")(i)
// }

// pub fn handle_command(command: String) -> Result<String, Error> {
//     log::trace!("handle command: {}", command);
//     let cmd = parse_line(&command)?;
//     match cmd {
//         Command::Person => {
//             log::debug!("got {:?}", cmd);
//         }
//     }
//     todo!("handle_command")
// }

// pub fn parse_line(line: &str) -> Result<Command, Error> {
//     log::trace!("parse line: {}", line);
//     let res = map(person, |_: &str| Command::Person)(line);
//     match res {
//         Ok((_, cmd)) => Ok(cmd),
//         Err(err) => Err(Error::Unknown(format!("{}", err))),
//     }
// }

// #[derive(Debug, Clone)]
// pub enum Command {
//     Person,
// }

use crate::{
    models::{self, AddPersonRequest, Person},
    ports::{
        data::{DataRepository, DataService},
        person::{PersonRepository, PersonService},
        LengthLogService, ServiceError,
    },
};

#[derive(Clone)]
pub struct Service<PR, DR> {
    person_repo: PR,
    data_repo: DR,
}

impl<PR, DR> Service<PR, DR> {
    pub fn new(person_repo: PR, data_repo: DR) -> Self {
        log::trace!("creating Service ...");
        Self {
            person_repo,
            data_repo,
        }
    }
}
impl<PR, DR> PersonService for Service<PR, DR>
where
    PR: PersonRepository,
{
    fn add_person(&self, req: models::AddPersonRequest) -> Result<Person, models::AddPersonError> {
        let person: Person = req.into();
        log::trace!(
            "adding person '{}' with date = {:?}",
            person.name(),
            person.start_date()
        );

        self.person_repo.save(&person)?;
        Ok(person)
    }
}

impl<PR, DR> DataService for Service<PR, DR>
where
    DR: DataRepository,
{
    fn add_datapoint(
        &self,
        req: models::datapoint::AddDatapointRequest,
    ) -> Result<(), ServiceError> {
        todo!()
    }
}

impl<PR, DR> LengthLogService for Service<PR, DR>
where
    PR: PersonRepository,
    DR: DataRepository,
{
    fn list_persons(&self) -> Result<Vec<Person>, ServiceError> {
        todo!()
    }
    fn save(&self) -> miette::Result<()> {
        self.person_repo.dump()?;
        self.data_repo.dump()?;
        Ok(())
    }
}
//     pub fn add_person(&self, name: String, start_date: Option<String>) -> Result<(), AppError> {
//         log::trace!("adding person '{}' with date = {:?}", name, start_date);
//
//         let start_date = if let Some(start_date_str) = start_date {
//             Some(NaiveDate::from_str(&start_date_str)?)
//         } else {
//             None
//         };
//         let person = Person::with_name_and_start_date(name, start_date);
//         self.person_service.save(person)?;
//         Ok(())
//     }
//
//     pub fn list_persons(&self) -> Result<Vec<models::Person>, AppError> {
//         Ok(self.person_service.get_all()?)
//     }
//
//     pub fn add_data(&self, name: &str, date: Option<String>, data: f64) -> Result<(), AppError> {
//         log::trace!(
//             "adding datapoint for person '{}' with date = {:?}",
//             name,
//             date
//         );
//         let id = self.person_service.get_id_by_name(name)?;
//         dbg!(&id);
//         let date = if let Some(date_str) = date {
//             NaiveDate::from_str(&date_str)?
//         } else {
//             Local::now().naive_local().date()
//         };
//         self.data_service.save(&id, date, data)?;
//         Ok(())
//     }
// }

#[derive(Debug)]
pub enum AppError {
    BadDate(chrono::ParseError),
    ServiceError(ServiceError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadDate(_) => f.write_str("bad date"),
            Self::ServiceError(_) => f.write_str("ServiceError"),
        }
    }
}

impl From<ServiceError> for AppError {
    fn from(value: ServiceError) -> Self {
        Self::ServiceError(value)
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(value: chrono::ParseError) -> Self {
        Self::BadDate(value)
    }
}
