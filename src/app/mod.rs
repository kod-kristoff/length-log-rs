use crate::error::Error;
use nom::{bytes::complete::tag, combinator::map, IResult};

fn person(i: &str) -> IResult<&str, &str> {
    tag("person")(i)
}

pub fn handle_command(command: String) -> Result<String, Error> {
    log::trace!("handle command: {}", command);
    let cmd = parse_line(&command)?;
    match cmd {
        Command::Person => {
            log::debug!("got {:?}", cmd);
        }
    }
    todo!("handle_command")
}

pub fn parse_line(line: &str) -> Result<Command, Error> {
    log::trace!("parse line: {}", line);
    let res = map(person, |_: &str| Command::Person)(line);
    match res {
        Ok((_, cmd)) => Ok(cmd),
        Err(err) => Err(Error::Unknown(format!("{}", err))),
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Person,
}

pub struct App {}

impl App {
    pub fn new() -> Self {
        log::trace!("creating App ...");
        Self {}
    }
pub fn add_person(&self, name: &str, date: Option<&str>) {
    log::trace!("adding person '{}' with date = {:?}", name, date);
}
}
