

use length_log_core::App;
use length_log_core_impl::services::PolarsPersonService;
use length_log_repl::run_repl;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .filter(Some("rustyline"), log::LevelFilter::Warn)
        .init();

    let person_service = PolarsPersonService::new_shared();
    let app = App::new(person_service);
    if let Err(err) = run_repl(app) {
        log::error!("{}", err);
        eprintln!("Error occurred: {:?}", err);
        std::process::exit(1);
    }
}
