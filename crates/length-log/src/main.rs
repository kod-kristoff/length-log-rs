use std::fs;

use directories::ProjectDirs;
use length_log_core::App;
use length_log_core_impl::services::{PolarsDataService, PolarsPersonService};
use length_log_repl::run_repl;
use miette::IntoDiagnostic;

fn main() -> miette::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .filter(Some("rustyline"), log::LevelFilter::Warn)
        .init();

    let length_log_dirs = ProjectDirs::from("casa", "kristoff", "length-log").unwrap();
    let data_dir = length_log_dirs.data_dir();
    let autosave = true;
    fs::create_dir_all(data_dir).into_diagnostic()?;
    let person_service =
        PolarsPersonService::load_or_create(data_dir.join("persons.csv")).into_diagnostic()?;
    let data_service =
        PolarsDataService::load_or_create(data_dir.join("data.parquet")).into_diagnostic()?;
    let app = App::new(person_service.clone(), data_service.clone());
    let res = run_repl(app);
    if autosave {
        person_service.dump()?;
        data_service.dump()?;
    }
    if let Err(err) = res {
        log::error!("{}", err);
        eprintln!("Error occurred: {:?}", err);
        std::process::exit(1);
    }
    Ok(())
}
