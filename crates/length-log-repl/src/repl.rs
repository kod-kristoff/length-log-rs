use crate::Error;
use chrono::NaiveDate;
use length_log_core::{
    models::{datapoint::AddDatapointRequest, AddPersonRequest, PersonName},
    ports::LengthLogService,
};
use miette::IntoDiagnostic;
use rustyline::{
    error::ReadlineError,
    history::{FileHistory, History},
    Config, Editor,
};
use std::{
    ffi::OsString,
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};

mod flags;

pub struct ReplApp<S> {
    length_log_service: S,
    repl: Editor<(), FileHistory>,
    history_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ReplAppConfig {
    pub history_path: PathBuf,
}

impl<S> ReplApp<S> {
    pub fn new(
        length_log_service: S,
        ReplAppConfig { history_path }: ReplAppConfig,
    ) -> miette::Result<Self> {
        log::debug!("creating ReplApp=");
        let config = Config::builder()
            .max_history_size(1000)
            .unwrap()
            .auto_add_history(true)
            .build();
        let mut history = FileHistory::new();
        if let Err(err) = history.load(&history_path) {
            log::warn!("could not load command history, err = {:?}", err);
        }
        let repl = Editor::<(), _>::with_history(config, history).into_diagnostic()?;

        Ok(Self {
            length_log_service,
            repl,
            history_path,
        })
    }
}
impl<S> ReplApp<S>
where
    S: LengthLogService,
{
    pub fn shut_down(&mut self) -> miette::Result<()> {
        self.length_log_service.save()?;
        self.repl
            .save_history(&self.history_path)
            .into_diagnostic()?;
        Ok(())
    }
    pub fn run(&mut self) -> miette::Result<()> {
        log::debug!("running ReplApp=");
        loop {
            let readline = self.repl.readline(">> ");
            match readline {
                Ok(line) => {
                    log::trace!("Line: {:?}", line);
                    match respond(&self.length_log_service, &line) {
                        Ok(quit) => {
                            log::trace!("command succeeded");

                            if quit {
                                log::info!("quitting ...");
                                break;
                            }
                        }
                        Err(err) => {
                            log::error!("error: {:?}", err);
                            println!("command '{}' failed with error: {}", line, err);
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    log::info!("CTRL-C pressed");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    log::info!("CTRL-D pressed");
                    break;
                }
                Err(err) => {
                    log::error!("An error occurred: {:?}", err);
                    break;
                }
            }
        }
        self.shut_down()?;
        Ok(())
    }
}

fn respond<S>(service: &S, line: &str) -> Result<bool, Error>
where
    S: LengthLogService,
{
    let args = shlex::split(line).ok_or_else(|| Error::InvalidQuoting(line.to_string()))?;
    let flags = flags::Repl::from_vec(args.iter().map(OsString::from).collect())
        .map_err(|e| Error::Unknown(e.to_string()))?;
    match flags.subcommand {
        flags::ReplCmd::AddPerson(flags::AddPerson { name, start_date }) => {
            log::trace!("adding person ...");
            log::trace!("name = {}", name);
            log::trace!("start_date = {:?}", start_date);
            let name = PersonName::new(&name)?;
            let req = AddPersonRequest::new(name, start_date);
            if let Err(err) = service.add_person(req) {
                log::error!("error adding person: err={:?}", err);
                eprintln!("Error adding person: {}", err);
            }
        }
        flags::ReplCmd::ListPersons(_) => match service.list_persons() {
            Ok(persons) => println!("{:#?}", persons),
            Err(err) => {
                log::error!("error adding person: err={:?}", err);
                eprintln!("Error adding person: {}", err);
            }
        },
        flags::ReplCmd::Add(flags::Add { name, data, date }) => {
            log::trace!("adding data {} for person {} ...", data, name);
            let name = PersonName::new(&name)?;
            // let date = if let Some(date) = date {
            //     Some(NaiveDate::from_str(&date).map_err(|err| Error::Unknown(err.to_string()))?)
            // } else {
            //     None
            // };
            let req = AddDatapointRequest::new(name, date, data);
            if let Err(err) = service.add_datapoint(req) {
                log::error!("error adding person: err={:?}", err);
                eprintln!("Error adding person: {}", err);
            }
        }
        flags::ReplCmd::Quit(_) => {
            writeln!(std::io::stdout(), "Exiting ...")?;
            std::io::stdout().flush()?;
            return Ok(true);
        }
    }

    Ok(false)
}
