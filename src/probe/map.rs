// crisp-status-local
//
// Crisp Status local probe relay
// Copyright: 2018, Crisp IM SAS
// License: Mozilla Public License v2.0 (MPL v2.0)

use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use serde_json;

use std::convert::TryFrom;
use std::io::Read;
use std::thread;
use std::time::Duration;

use super::replica::ReplicaURL;
use super::report::{
    generate_url as report_generate_url, REPORT_HTTP_CLIENT_TIMEOUT,
    REPORT_HTTP_HEADER_AUTHORIZATION, REPORT_HTTP_HEADER_USERAGENT,
};
use crate::utilities::chunk::Decoder as ChunkDecoder;

const RETRY_ACQUIRE_TIMES: u8 = 2;
const RETRY_ACQUIRE_AFTER_SECONDS: u64 = 5;

#[derive(Deserialize)]
pub struct MapFromResponse {
    pub data: Map,
}

#[derive(Deserialize)]
pub struct Map {
    pub date: Option<u64>,
    pub metrics: Option<MapMetrics>,
    pub services: Vec<MapService>,
}

#[derive(Deserialize)]
pub struct MapMetrics {
    pub poll: MapMetricsPoll,
    pub push: MapMetricsPush,
    pub local: MapMetricsLocal,
}

#[derive(Deserialize)]
pub struct MapMetricsPoll {
    pub retry: u8,
    pub delay_dead: u64,
    pub delay_sick: u64,
}

#[derive(Deserialize)]
pub struct MapMetricsPush {
    pub delay_dead: u64,
    pub system_cpu_sick_above: f32,
    pub system_ram_sick_above: f32,
}

#[derive(Deserialize)]
pub struct MapMetricsLocal {
    pub retry: u8,
    pub delay_dead: u64,
    pub delay_sick: u64,
}

#[derive(Deserialize)]
pub struct MapService {
    pub id: String,
    pub nodes: Vec<MapServiceNode>,
}

#[derive(Deserialize)]
pub struct MapServiceNode {
    pub id: String,
    pub replicas: Option<Vec<ReplicaURL>>,
    pub http: Option<MapServiceNodeHTTP>,
}

#[derive(Deserialize)]
pub struct MapServiceNodeHTTP {
    pub status: Option<MapServiceNodeHTTPStatus>,
    pub body: Option<MapServiceNodeHTTPBody>,
}

#[derive(Deserialize)]
pub struct MapServiceNodeHTTPStatus {
    pub healthy_above: Option<u16>,
    pub healthy_below: Option<u16>,
}

#[derive(Deserialize)]
pub struct MapServiceNodeHTTPBody {
    pub healthy_match: Option<String>,
}

#[derive(Debug)]
pub enum MapError {
    FailedRequest,
    NotAuthorized,
    InvalidStatus,
    InvalidData,
    ExhaustedAttempts,
}

pub fn acquire(map: &mut Map) -> Result<(), MapError> {
    // Attempt to acquire (first attempt)
    acquire_attempt(map, 0)
}

fn acquire_attempt(map: &mut Map, attempt: u8) -> Result<(), MapError> {
    info!("running acquire attempt #{}", attempt);

    match acquire_request(map) {
        Ok(_) => Ok(()),
        Err(MapError::NotAuthorized) => Err(MapError::NotAuthorized),
        Err(_) => {
            let next_attempt = attempt + 1;

            if next_attempt > RETRY_ACQUIRE_TIMES {
                Err(MapError::ExhaustedAttempts)
            } else {
                warn!(
                    "acquire attempt #{} failed, will retry after delay",
                    attempt
                );

                // Retry after delay
                thread::sleep(Duration::from_secs(RETRY_ACQUIRE_AFTER_SECONDS));

                acquire_attempt(map, next_attempt)
            }
        }
    }
}

fn acquire_request(map: &mut Map) -> Result<(), MapError> {
    // Generate probe path
    let mut probe_path = String::from("probes/local");

    if let Some(date) = map.date {
        probe_path.push_str("?since=");
        probe_path.push_str(&date.to_string());
    }

    let probe_url = report_generate_url(&probe_path);

    debug!("generated probes url: {}", &probe_url);

    // Generate request URI
    let request_uri = Uri::try_from(probe_url.as_str()).expect("invalid probe request uri");

    // Acquire probe response
    let mut response_body = Vec::new();

    let response = Request::new(&request_uri)
        .connect_timeout(Some(REPORT_HTTP_CLIENT_TIMEOUT))
        .read_timeout(Some(REPORT_HTTP_CLIENT_TIMEOUT))
        .write_timeout(Some(REPORT_HTTP_CLIENT_TIMEOUT))
        .method(Method::GET)
        .header("User-Agent", &*REPORT_HTTP_HEADER_USERAGENT)
        .header("Authorization", &*REPORT_HTTP_HEADER_AUTHORIZATION)
        .send(&mut response_body);

    // Acquire items
    match response {
        Ok(response) => {
            let status_code = u16::from(response.status_code());

            debug!("acquired probe map with status: {}", status_code);

            // Parse JSON result
            match status_code {
                200 => {
                    // Status is: 'OK'

                    // Read response headers
                    let headers = response.headers();

                    let (content_type, transfer_encoding) = (
                        headers
                            .get("Content-Type")
                            .map(|value| value.to_owned())
                            .unwrap_or("".to_string()),
                        headers
                            .get("Transfer-Encoding")
                            .map(|value| value.to_owned())
                            .unwrap_or("identity".to_string()),
                    );

                    // Validate response
                    if content_type.starts_with("application/json")
                        && (transfer_encoding == "identity" || transfer_encoding == "chunked")
                        && !response_body.is_empty()
                    {
                        // Decode body using an appropriate decoding method
                        response_body = if transfer_encoding == "chunked" {
                            // Decode chunked HTTP encoding
                            let mut response_body_decoded = Vec::new();

                            let mut chunked_decoder = ChunkDecoder::new(response_body.as_slice());

                            chunked_decoder.read_to_end(&mut response_body_decoded).ok();

                            response_body_decoded
                        } else {
                            // Return identity
                            response_body
                        };

                        match serde_json::from_slice::<MapFromResponse>(&response_body) {
                            Ok(response_json) => {
                                info!("acquired probe map with changes");

                                // Alter map object
                                map.date = response_json.data.date;
                                map.services = response_json.data.services;
                                map.metrics = response_json.data.metrics;

                                Ok(())
                            }
                            Err(err) => {
                                warn!("got invalid data for probe map acquire: {}", err);

                                Err(MapError::InvalidData)
                            }
                        }
                    } else {
                        warn!(
                            "received headers not expected for probe map acquire: '{}' / '{}'",
                            content_type, transfer_encoding
                        );

                        Err(MapError::InvalidData)
                    }
                }
                304 => {
                    // Status is: 'Not Modified'
                    debug!("acquired probe map with no changes");

                    Ok(())
                }
                _ => {
                    warn!(
                        "got invalid status code for probe map acquire: {}",
                        status_code
                    );

                    // Invalid token?
                    if status_code == 401 {
                        // Status is: 'Unauthorized'
                        error!("[important] your reporter token is invalid, please update it");

                        Err(MapError::NotAuthorized)
                    } else {
                        Err(MapError::InvalidStatus)
                    }
                }
            }
        }
        Err(err) => {
            warn!("failed acquiring probe map: {}", err);

            Err(MapError::FailedRequest)
        }
    }
}
