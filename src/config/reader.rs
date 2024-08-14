// crisp-status-local
//
// Crisp Status local probe relay
// Copyright: 2018, Crisp IM SAS
// License: Mozilla Public License v2.0 (MPL v2.0)

use envsubst::substitute;
use std::{collections::HashMap, env, fs};
use toml;

use super::config::*;
use crate::APP_ARGS;

pub struct ConfigReader;

impl ConfigReader {
    pub fn make() -> Config {
        debug!("reading config file: {}", &APP_ARGS.config);

        // Read configuration
        let mut conf = fs::read_to_string(&APP_ARGS.config).expect("cannot read config file");

        debug!("read config file: {}", &APP_ARGS.config);

        // Replace environment variables
        let environment = env::vars().collect::<HashMap<String, String>>();

        conf = substitute(&conf, &environment).expect("cannot substitute environment variables");

        // Parse configuration
        let config = toml::from_str(&conf).expect("syntax error in config file");

        config
    }
}
