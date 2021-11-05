#![feature(path_file_prefix)]

use rocket::launch;
use structopt::StructOpt;

use crate::race::RaceService;

mod api;
mod config;
mod race;

#[derive(Debug, StructOpt)]
struct Cli {
    /// config file
    #[structopt(long = "config-file", short = "c", default_value = "config.yaml")]
    config_file: String,
}

#[launch]
fn rocket() -> _ {
    std::env::set_var("RUST_LOG", "error,races=info");
    env_logger::init();

    let args = Cli::from_args();

    let config: config::Config = confy::load_path(std::path::Path::new(&args.config_file)).unwrap();

    let race_service = RaceService::new(config.races_dir, config.archived_dir);

    api::init().manage(race_service)
}
