// crisp-status-local
//
// Crisp Status local probe relay
// Copyright: 2018, Crisp IM SAS
// License: Mozilla Public License v2.0 (MPL v2.0)

use base64::engine::general_purpose::STANDARD as base64_encoder;
use base64::Engine;
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use serde_json;

use std::convert::TryFrom;
use std::io;
use std::thread;
use std::time::Duration;

use super::map::{MapService, MapServiceNode};
use super::replica::ReplicaURL;
use super::status::Status;

use crate::APP_CONF;

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
    pub static ref REPORT_HTTP_HEADER_USERAGENT: String =
        format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    pub static ref REPORT_HTTP_HEADER_AUTHORIZATION: String = format!(
        "Basic {}",
        base64_encoder.encode(&format!(":{}", APP_CONF.report.token))
    );
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
    // Generate report URL
    let report_url = generate_url(&format!("report/{}/{}", &service.id, &node.id));

    debug!("generated report url: {}", &report_url);

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
    let request_uri = Uri::try_from(report_url.as_str()).expect("invalid status request uri");

    // Acquire report response
    let mut response_sink = io::sink();

    let response = Request::new(&request_uri)
        .connect_timeout(Some(REPORT_HTTP_CLIENT_TIMEOUT))
        .read_timeout(Some(REPORT_HTTP_CLIENT_TIMEOUT))
        .write_timeout(Some(REPORT_HTTP_CLIENT_TIMEOUT))
        .method(Method::POST)
        .header("User-Agent", &*REPORT_HTTP_HEADER_USERAGENT)
        .header("Authorization", &*REPORT_HTTP_HEADER_AUTHORIZATION)
        .header("Content-Type", "application/json")
        .header("Content-Length", &payload_json.len())
        .body(&payload_json)
        .send(&mut response_sink);

    match response {
        Ok(response) => {
            let status_code = response.status_code();

            if status_code.is_success() {
                debug!("reported to probe url: {}", report_url);

                Ok(())
            } else {
                debug!(
                    "could not report to probe url: {} (got status code: {})",
                    report_url, status_code
                );

                Err(())
            }
        }
        Err(err) => {
            warn!(
                "failed reporting to probe url: {} because: {}",
                report_url, err
            );

            Err(())
        }
    }
}
