use crate::Error;
use length_log_core::App;
use rustyline::{
    error::ReadlineError,
    history::{FileHistory, History},
    Config,
};
use std::{ffi::OsString, io::Write, path::Path};

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
            match app.list_data() {
                Ok(data) => println!("{:#?}", data),
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
