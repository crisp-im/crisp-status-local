// crisp-status-local
//
// Crisp Status local probe relay
// Copyright: 2018, Crisp IM SARL
// License: Mozilla Public License v2.0 (MPL v2.0)

use reqwest::{Client, RedirectPolicy};
use reqwest::header::{Headers, UserAgent, Authorization, Basic};

use std::time::Duration;

use APP_CONF;

const REPORT_HTTP_CLIENT_TIMEOUT: u64 = 20;

lazy_static! {
    pub static ref REPORT_HTTP_CLIENT: Client = Client::builder()
        .timeout(Duration::from_secs(REPORT_HTTP_CLIENT_TIMEOUT))
        .gzip(true)
        .redirect(RedirectPolicy::none())
        .enable_hostname_verification()
        .default_headers(make_http_client_headers())
        .build()
        .unwrap();
}

pub fn generate_url(path: &str) -> String {
    format!("{}/{}", &APP_CONF.report.endpoint, path)
}

fn make_http_client_headers() -> Headers {
    let mut headers = Headers::new();

    headers.set(UserAgent::new(format!(
        "{}/{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    )));

    headers.set(Authorization(Basic {
        username: "".to_owned(),
        password: Some(APP_CONF.report.token.to_owned()),
    }));

    headers
}
