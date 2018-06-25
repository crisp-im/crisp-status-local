// crisp-status-local
//
// Crisp Status local probe relay
// Copyright: 2018, Crisp IM SARL
// License: Mozilla Public License v2.0 (MPL v2.0)

use super::defaults;

#[derive(Deserialize)]
pub struct Config {
    pub server: ConfigServer,
}

#[derive(Deserialize)]
pub struct ConfigServer {
    #[serde(default = "defaults::server_log_level")]
    pub log_level: String,
}
