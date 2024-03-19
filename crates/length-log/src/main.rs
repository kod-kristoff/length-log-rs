use length_log_core::App;
use length_log_repl::run_repl;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .filter(Some("rustyline"), log::LevelFilter::Warn)
        .init();

    let app = App::new();
    if let Err(err) = run_repl(app) {
        log::error!("{}", err);
        eprintln!("Error occurred: {:?}", err);
        std::process::exit(1);
    }
}
