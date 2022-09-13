mod app;
mod error;
use error::Error;
use rustyline::error::ReadlineError;
use std::io::Write;
fn main() -> rustyline::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .filter(Some("rustyline"), log::LevelFilter::Warn)
        .init();

    let mut repl = rustyline::Editor::<()>::new()?;
    loop {
        let readline = repl.readline(">> ");
        match readline {
            Ok(line) => {
                log::trace!("Line: {:?}", line);
                match respond(&line) {
                    Ok(quit) => {
                        log::trace!("command succedded");

                        if quit {
                            log::info!("quiting ...");
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
                log::error!("An error occured: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

fn respond(line: &str) -> Result<bool, Error> {
    let args = shlex::split(line).ok_or_else(|| Error::InvalidQuoting(line.to_string()))?;
    let matches = cli()
        .try_get_matches_from(&args)
        .map_err(|e| Error::Unknown(e.to_string()))?;
    match matches.subcommand() {
        Some(("person", matches)) => {
            log::trace!("got person command");
            match matches.subcommand() {
                Some(("add", submatches)) => {
                    log::trace!("adding person ...");
                    let name = submatches.value_of("name").expect("required");
                    log::trace!("name = {}", name);
                    let date = submatches.value_of("date");
                    log::trace!("date = {:?}", date);
                    app::add_person(name, date);
                }
                Some((name, _matches)) => unimplemented!("{}", name),
                None => unreachable!("subcommand required"),
            }
            writeln!(std::io::stdout(), "Pong")?;
            std::io::stdout().flush()?;
        }
        Some(("quit", _matches)) => {
            writeln!(std::io::stdout(), "Exiting ...")?;
            std::io::stdout().flush()?;
            return Ok(true);
        }
        Some((name, _matches)) => unimplemented!("{}", name),
        None => unreachable!("subcommand required"),
    }

    Ok(false)
}

fn cli() -> clap::Command<'static> {
    use clap::{Arg, Command};
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
    // strip out name/version
    const APPLET_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    Command::new("length-log")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("APPLET")
        .subcommand_help_heading("APPLETS")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("person")
                .about("Handle person")
                .help_template(APPLET_TEMPLATE)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("add")
                        .arg(Arg::new("name").required(true).takes_value(true))
                        .arg(Arg::new("date").takes_value(true)),
                ),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .about("Quit the REPL")
                .help_template(APPLET_TEMPLATE),
        )
}
