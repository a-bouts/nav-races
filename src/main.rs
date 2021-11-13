#![feature(path_file_prefix)]

use rocket::launch;
use structopt::StructOpt;
use crate::polar::PolarService;

use crate::race::RaceService;

mod api;
mod config;
mod race;
mod polar;

#[derive(Debug, StructOpt)]
struct Cli {
    /// config file
    #[structopt(long = "config-file", short = "c", default_value = "config.yaml")]
    config_file: String,
}

#[launch]
fn rocket() -> _ {
    std::env::var("RUST_LOG").map_err(|_| {
        std::env::set_var("RUST_LOG", "error,races=info");
    }).unwrap_or_default();
    env_logger::init();

    let args = Cli::from_args();

    let config: config::Config = confy::load_path(std::path::Path::new(&args.config_file)).unwrap();

    let race_service = RaceService::new(config.races_dir, config.archived_dir);

    let polar_service = PolarService::new(config.polars);

    api::init().manage(race_service).manage(polar_service)
}
