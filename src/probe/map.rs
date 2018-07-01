// crisp-status-local
//
// Crisp Status local probe relay
// Copyright: 2018, Crisp IM SARL
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::thread;
use std::time::Duration;

use reqwest::StatusCode;

use super::report::{REPORT_HTTP_CLIENT, generate_url as report_generate_url};
use super::replica::ReplicaURL;

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

    debug!("generated probes url: {}", &probe_path);

    // Acquire items
    match REPORT_HTTP_CLIENT
        .get(&report_generate_url(&probe_path))
        .send() {
        Ok(mut response_inner) => {
            let status = response_inner.status();

            debug!("acquired probe map with status: {}", status.as_u16());

            // Parse JSON result
            match status {
                StatusCode::Ok => {
                    match response_inner.json::<MapFromResponse>() {
                        Ok(response_json) => {
                            info!("acquired probe map with changes");

                            // Alter map object
                            map.date = response_json.data.date;
                            map.services = response_json.data.services;

                            Ok(())
                        }
                        Err(err) => {
                            warn!("got invalid data for probe map acquire: {}", err);

                            Err(MapError::InvalidData)
                        }
                    }
                }
                StatusCode::NotModified => {
                    debug!("acquired probe map with no changes");

                    Ok(())
                }
                _ => {
                    warn!(
                        "got invalid status code for probe map acquire: {}",
                        status.as_u16()
                    );

                    // Invalid token?
                    if status == StatusCode::Unauthorized {
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
