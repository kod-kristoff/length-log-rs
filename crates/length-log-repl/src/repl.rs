use crate::Error;
use length_log_core::{
    models::{DataPoint, Person},
    App,
};
use rustyline::{
    error::ReadlineError,
    history::{FileHistory, History},
    Config,
};
use std::{
    collections::HashMap,
    ffi::OsString,
    io::{self, Write},
    path::Path,
};

mod flags;

pub fn run_repl(app: App) -> rustyline::Result<()> {
    log::debug!("running repl app=");
    let config = Config::builder()
        .max_history_size(1000)
        .unwrap()
        .auto_add_history(true)
        .build();
    let mut history = FileHistory::new();
    let history_path = Path::new("./data/history");
    if let Err(err) = history.load(history_path) {
        log::warn!("could not load command history, err = {:?}", err);
    }
    let mut repl = rustyline::Editor::<(), _>::with_history(config, history)?;
    loop {
        let readline = repl.readline(">> ");
        match readline {
            Ok(line) => {
                log::trace!("Line: {:?}", line);
                match respond(&app, &line) {
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
    repl.save_history(history_path).unwrap();
    Ok(())
}

fn respond(app: &App, line: &str) -> Result<bool, Error> {
    let args = shlex::split(line).ok_or_else(|| Error::InvalidQuoting(line.to_string()))?;
    let flags = flags::Repl::from_vec(args.iter().map(OsString::from).collect())
        .map_err(|e| Error::Unknown(e.to_string()))?;
    match flags.subcommand {
        flags::ReplCmd::AddPerson(flags::AddPerson { name, start_date }) => {
            log::trace!("adding person ...");
            log::trace!("name = {}", name);
            log::trace!("start_date = {:?}", start_date);
            if let Err(err) = app.add_person(name, start_date) {
                log::error!("error adding person: err={:?}", err);
                eprintln!("Error adding person: {}", err);
            }
        }
        flags::ReplCmd::ListPersons(_) => match app.list_persons() {
            Ok(persons) => println!("{:#?}", persons),
            Err(err) => {
                log::error!("error adding person: err={:?}", err);
                eprintln!("Error adding person: {}", err);
            }
        },
        flags::ReplCmd::Add(flags::Add { name, data, date }) => {
            log::trace!("adding data {} for person {} ...", data, name);
            if let Err(err) = app.add_data(&name, date, data) {
                log::error!("error adding person: err={:?}", err);
                eprintln!("Error adding person: {}", err);
            }
        }
        flags::ReplCmd::ListData(_) => {
            log::trace!("listing all data");
            let id_person_map = match app.list_persons() {
                Ok(persons) => {
                    let mut id_persons = HashMap::new();
                    for person in persons {
                        id_persons.insert(person.id, person.name);
                    }
                    id_persons
                }
                Err(err) => {
                    log::error!("error adding person: err={:?}", err);
                    eprintln!("Error adding person: {}", err);
                    return Ok(false);
                }
            };
            match app.list_data() {
                Ok(data) => {
                    for data_point in data {
                        println!("{:#?}", data_point)
                    }
                }
                Err(err) => {
                    log::error!("error adding person: err={:?}", err);
                    eprintln!("Error adding person: {}", err);
                }
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

fn print_data_list(
    data_points: &[DataPoint],
    id_person_map: HashMap<String, Person>,
) -> Result<(), io::Error> {
}
