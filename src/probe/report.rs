// crisp-status-local
//
// Crisp Status local probe relay
// Copyright: 2018, Crisp IM SARL
// License: Mozilla Public License v2.0 (MPL v2.0)

use reqwest::{Client, StatusCode, RedirectPolicy};
use reqwest::header::{Headers, UserAgent, Authorization, Basic};

use std::thread;
use std::time::Duration;

use super::map::{MapService, MapServiceNode};
use super::replica::ReplicaURL;
use super::status::Status;

use APP_CONF;

const REPORT_HTTP_CLIENT_TIMEOUT: u64 = 20;
const RETRY_STATUS_TIMES: u8 = 2;
const RETRY_STATUS_AFTER_SECONDS: u64 = 5;

#[derive(Serialize)]
struct ReportPayload<'a> {
    replica_id: &'a str,
    health: &'a str,
    interval: u64,
}

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

pub fn status(
    service: &MapService,
    node: &MapServiceNode,
    replica: &ReplicaURL,
    status: &Status,
    interval: u64,
) -> Result<(), ()> {
    // Attempt to acquire (first attempt)
    status_attempt(service, node, replica, status, interval, 0)
}

fn status_attempt(
    service: &MapService,
    node: &MapServiceNode,
    replica: &ReplicaURL,
    status: &Status,
    interval: u64,
    attempt: u8,
) -> Result<(), ()> {
    info!(
        "running status report attempt #{} on #{}:#{}:[{:?}]",
        attempt,
        service.id,
        node.id,
        replica
    );

    match status_request(service, node, replica, status, interval) {
        Ok(_) => Ok(()),
        Err(_) => {
            let next_attempt = attempt + 1;

            if next_attempt > RETRY_STATUS_TIMES {
                Err(())
            } else {
                warn!(
                    "status report attempt #{} failed on #{}:#{}:[{:?}], will retry after delay",
                    attempt,
                    service.id,
                    node.id,
                    replica
                );

                // Retry after delay
                thread::sleep(Duration::from_secs(RETRY_STATUS_AFTER_SECONDS));

                status_attempt(service, node, replica, status, interval, next_attempt)
            }
        }
    }
}

fn status_request(
    service: &MapService,
    node: &MapServiceNode,
    replica: &ReplicaURL,
    status: &Status,
    interval: u64,
) -> Result<(), ()> {
    // Generate report path
    let report_path = format!("report/{}/{}", &service.id, &node.id);

    debug!("generated report url: {}", &report_path);

    // Acquire report response
    let response = REPORT_HTTP_CLIENT
        .post(&generate_url(&report_path))
        .json(&ReportPayload {
            replica_id: replica.get_raw(),
            health: status.as_str(),
            interval: interval,
        })
        .send();

    match response {
        Ok(response_inner) => {
            let status = response_inner.status();

            if status == StatusCode::Ok {
                debug!("reported to probe path: {}", report_path);

                Ok(())
            } else {
                debug!(
                    "could not report to probe path: {} (got status: {})",
                    report_path,
                    status
                );

                Err(())
            }
        }
        Err(err) => {
            warn!(
                "failed reporting to probe path: {} because: {}",
                report_path,
                err
            );

            Err(())
        }
    }
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
