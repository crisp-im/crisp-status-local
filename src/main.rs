// crisp-status-local
//
// Crisp Status local probe relay
// Copyright: 2018, Crisp IM SARL
// License: Mozilla Public License v2.0 (MPL v2.0)

#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate toml;

mod config;

use std::ops::Deref;
use std::str::FromStr;

use clap::{App, Arg};
use log::LevelFilter;

use config::config::Config;
use config::logger::ConfigLogger;
use config::reader::ConfigReader;

struct AppArgs {
    config: String,
}

lazy_static! {
    static ref APP_ARGS: AppArgs = make_app_args();
    static ref APP_CONF: Config = ConfigReader::make();
}

fn make_app_args() -> AppArgs {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Path to configuration file")
                .default_value("./config.cfg")
                .takes_value(true),
        )
        .get_matches();

    // Generate owned app arguments
    AppArgs { config: String::from(matches.value_of("config").expect("invalid config value")) }
}

fn ensure_states() {
    // Ensure all statics are valid (a `deref` is enough to lazily initialize them)
    APP_ARGS.deref();
    APP_CONF.deref();
}

fn main() {
    // Initialize shared logger
    let _logger = ConfigLogger::init(LevelFilter::from_str(&APP_CONF.server.log_level).expect(
        "invalid log level",
    ));

    info!("starting up");

    // Ensure all states are bound
    ensure_states();

    // TODO

    error!("could not start");
}
