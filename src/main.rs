mod app;
mod error;
use rustyline::error::ReadlineError;

fn main() -> rustyline::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .filter(Some("rustyline"), log::LevelFilter::Warn)
        .init();

    let mut repl = rustyline::Editor::<()>::new()?;
    loop {
        let readline = repl.readline(">> ");
        match readline {
            Ok(command) => {
                log::trace!("Line: {:?}", command);
                match app::handle_command(command.clone()) {
                    Ok(result) => {
                        log::trace!("command succedded");
                        println!("{}", result);
                    }
                    Err(err) => {
                        log::error!("error: {:?}", err);
                        eprintln!("command '{}' failed", command);
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
