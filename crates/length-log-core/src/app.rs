use std::str::FromStr;

use chrono::NaiveDate;
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

use crate::{models::Person, services::SharedPersonService};

#[derive(Clone)]
pub struct App {
    person_service: SharedPersonService,
}

impl App {
    pub fn new(person_service: SharedPersonService) -> Self {
        log::trace!("creating App ...");
        Self {
            person_service
        }
    }
    pub fn add_person(&self, name: String, start_date: Option<String>) {
        log::trace!("adding person '{}' with date = {:?}", name, start_date);
        let start_date = if let Some(start_date_str) = start_date {
            Some(NaiveDate::from_str(&start_date_str).unwrap())
        } else {
            None
        };
        let person = Person { id: String::new(), name, start_date};
        self.person_service.save(person).unwrap();
    }
}
