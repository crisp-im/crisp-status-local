// crisp-status-local
//
// Crisp Status local probe relay
// Copyright: 2018, Crisp IM SARL
// License: Mozilla Public License v2.0 (MPL v2.0)

pub fn server_log_level() -> String {
    "warn".to_string()
}

pub fn report_endpoint() -> String {
    "https://report.crisp.watch/v1".to_string()
}
