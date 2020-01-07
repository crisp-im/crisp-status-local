// crisp-status-local
//
// Crisp Status local probe relay
// Copyright: 2018, Crisp IM SARL
// License: Mozilla Public License v2.0 (MPL v2.0)

use base64;
use http_req::{
    request::{Method, Request},
    response::Headers,
    uri::Uri,
};
use serde_json;

use std::io;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use super::map::{MapService, MapServiceNode};
use super::replica::ReplicaURL;
use super::status::Status;

use APP_CONF;

pub const REPORT_HTTP_CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

const RETRY_STATUS_TIMES: u8 = 2;
const RETRY_STATUS_AFTER_SECONDS: u64 = 5;

#[derive(Serialize)]
struct ReportPayload<'a> {
    replica_id: &'a str,
    health: &'a str,
    interval: u64,
}

lazy_static! {
    static ref REPORT_HTTP_HEADER_USERAGENT: String =
        format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    static ref REPORT_HTTP_HEADER_AUTHORIZATION: String = format!(
        "Basic {}",
        base64::encode(&format!(":{}", APP_CONF.report.token))
    );
}

pub fn generate_url(path: &str) -> String {
    format!("{}/{}", &APP_CONF.report.endpoint, path)
}

pub fn make_status_request_headers(body_data: Option<&[u8]>) -> Headers {
    let mut headers = Headers::new();

    headers.insert("User-Agent", &*REPORT_HTTP_HEADER_USERAGENT);
    headers.insert("Authorization", &*REPORT_HTTP_HEADER_AUTHORIZATION);

    if let Some(body_data) = body_data {
        headers.insert("Content-Type", "application/json");
        headers.insert("Content-Length", &body_data.len());
    }

    headers
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
        attempt, service.id, node.id, replica
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
                    attempt, service.id, node.id, replica
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

    // Generate report payload
    let payload = ReportPayload {
        replica_id: replica.get_raw(),
        health: status.as_str(),
        interval: interval,
    };

    // Encore payload to string
    // Notice: fail hard if payload is invalid (it should never be)
    let payload_json = serde_json::to_vec(&payload).expect("invalid status request payload");

    // Generate request URI
    let request_uri =
        Uri::from_str(&generate_url(&report_path)).expect("invalid status request uri");

    // Acquire report response
    let mut response_sink = io::sink();

    let response = Request::new(&request_uri)
        .connect_timeout(Some(REPORT_HTTP_CLIENT_TIMEOUT))
        .read_timeout(Some(REPORT_HTTP_CLIENT_TIMEOUT))
        .write_timeout(Some(REPORT_HTTP_CLIENT_TIMEOUT))
        .method(Method::POST)
        .headers(make_status_request_headers(Some(&payload_json)))
        .body(&payload_json)
        .send(&mut response_sink);

    match response {
        Ok(response) => {
            let status_code = response.status_code();

            if status_code.is_success() {
                debug!("reported to probe path: {}", report_path);

                Ok(())
            } else {
                debug!(
                    "could not report to probe path: {} (got status code: {})",
                    report_path, status_code
                );

                Err(())
            }
        }
        Err(err) => {
            warn!(
                "failed reporting to probe path: {} because: {}",
                report_path, err
            );

            Err(())
        }
    }
}
