mod app;
mod error;
use rustyline::error::ReadlineError;
use error::Error;
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
        Some(("ping", _matches)) => {
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
    use clap::Command;
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

    Command::new("repl")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("APPLET")
        .subcommand_help_heading("APPLETS")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("ping")
                .about("Get a response")
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .about("Quit the REPL")
                .help_template(APPLET_TEMPLATE),
        )
}
