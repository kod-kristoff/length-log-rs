use std::fs;

use directories::ProjectDirs;
use length_log_core::Service;
use length_log_polars::{datapoints::PolarsDataRepo, persons::PolarsPersonRepo};
use length_log_repl::{ReplApp, ReplAppConfig};
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
    let persons_path = data_dir.join("persons.csv");
    let data_path = data_dir.join("data.parquet");
    let history_path = data_dir.join("history");

    let person_repo = if persons_path.exists() {
        PolarsPersonRepo::from_path(persons_path.as_path()).into_diagnostic()?
    } else {
        PolarsPersonRepo::with_path(persons_path)
    };
    let data_repo = if data_path.exists() {
        PolarsDataRepo::from_path(data_path).into_diagnostic()?
    } else {
        PolarsDataRepo::with_path(data_path)
    };
    let service = Service::new(person_repo, data_repo);
    let app_config = ReplAppConfig { history_path };
    let mut app = ReplApp::new(service, app_config)?;
    app.run()?;
    Ok(())
}
