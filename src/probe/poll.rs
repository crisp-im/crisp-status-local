// crisp-status-local
//
// Crisp Status local probe relay
// Copyright: 2018, Crisp IM SARL
// License: Mozilla Public License v2.0 (MPL v2.0)

use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use memmem::{Searcher, TwoWaySearcher};

use std::io::Read;
use std::net::{TcpStream, ToSocketAddrs};
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

use super::map::{MapMetrics, MapService, MapServiceNodeHTTP};
use super::replica::ReplicaURL;
use super::report::status as report_status;
use super::status::Status;
use crate::utilities::chunk::Decoder as ChunkDecoder;

const NODE_HTTP_HEALTHY_ABOVE: u16 = 200;
const NODE_HTTP_HEALTHY_BELOW: u16 = 400;
const RETRY_REPLICA_AFTER_MILLISECONDS: u64 = 200;

lazy_static! {
    static ref POLL_HTTP_HEADER_USERAGENT: String =
        format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

pub fn dispatch(services: &Vec<MapService>, metrics: &Option<MapMetrics>, interval: u64) {
    debug!("will dispatch polls");

    for service in services {
        debug!("scanning for polls in service: #{}", service.id);

        for node in &service.nodes {
            debug!("scanning for polls in service node: #{}", node.id);

            if let Some(ref replicas) = node.replicas {
                for replica in replicas {
                    let replica_status =
                        proceed_replica(&service.id, &node.id, replica, &node.http, metrics);

                    debug!("got replica status upon poll: {:?}", replica_status);

                    match report_status(service, node, replica, &replica_status, interval) {
                        Ok(_) => info!("reported replica status: {:?}", replica_status),
                        Err(_) => warn!("failed reporting replica status: {:?}", replica_status),
                    }
                }
            }
        }
    }

    info!("dispatched polls");
}

pub fn proceed_replica(
    service_id: &str,
    node_id: &str,
    replica: &ReplicaURL,
    http: &Option<MapServiceNodeHTTP>,
    metrics: &Option<MapMetrics>,
) -> Status {
    // Acquire number of times to retry
    let retry_times = if let &Some(ref metrics_inner) = metrics {
        metrics_inner.local.retry
    } else {
        2
    };

    // Attempt to acquire (first attempt)
    proceed_replica_attempt(service_id, node_id, replica, http, metrics, retry_times, 0)
}

fn proceed_replica_attempt(
    service_id: &str,
    node_id: &str,
    replica: &ReplicaURL,
    http: &Option<MapServiceNodeHTTP>,
    metrics: &Option<MapMetrics>,
    retry_times: u8,
    attempt: u8,
) -> Status {
    info!(
        "running replica scan attempt #{} on #{}:#{}:[{:?}]",
        attempt, service_id, node_id, replica
    );

    match proceed_replica_request(service_id, node_id, replica, http, metrics) {
        Status::Healthy => Status::Healthy,
        Status::Sick => Status::Sick,
        Status::Dead => {
            let next_attempt = attempt + 1;

            if next_attempt > retry_times {
                Status::Dead
            } else {
                warn!(
                    "replica scan attempt #{} failed on #{}:#{}:[{:?}], will retry after delay",
                    attempt, service_id, node_id, replica
                );

                // Retry after delay
                thread::sleep(Duration::from_millis(RETRY_REPLICA_AFTER_MILLISECONDS));

                proceed_replica_attempt(
                    service_id,
                    node_id,
                    replica,
                    http,
                    metrics,
                    retry_times,
                    next_attempt,
                )
            }
        }
    }
}

fn proceed_replica_request(
    service_id: &str,
    node_id: &str,
    replica: &ReplicaURL,
    http: &Option<MapServiceNodeHTTP>,
    metrics: &Option<MapMetrics>,
) -> Status {
    debug!(
        "scanning replica: #{}:#{}:[{:?}]",
        service_id, node_id, replica
    );

    let start_time = SystemTime::now();

    let is_up = match replica {
        &ReplicaURL::TCP(_, ref host, port) => proceed_replica_request_tcp(host, port, metrics),
        &ReplicaURL::HTTP(_, ref url) => proceed_replica_request_http(url, http, metrics),
        &ReplicaURL::HTTPS(_, ref url) => proceed_replica_request_http(url, http, metrics),
    };

    if is_up == true {
        // Probe reports as sick?
        if let Ok(duration_since) = SystemTime::now().duration_since(start_time) {
            if let &Some(ref metrics_inner) = metrics {
                if duration_since >= Duration::from_secs(metrics_inner.local.delay_sick) {
                    return Status::Sick;
                }
            }
        }

        Status::Healthy
    } else {
        Status::Dead
    }
}

fn proceed_replica_request_tcp(host: &str, port: u16, metrics: &Option<MapMetrics>) -> bool {
    let address_results = (host, port).to_socket_addrs();

    if let Ok(mut address) = address_results {
        if let Some(address_value) = address.next() {
            debug!("prober poll will fire for tcp target: {}", address_value);

            return match TcpStream::connect_timeout(&address_value, acquire_dead_timeout(metrics)) {
                Ok(_) => true,
                Err(_) => false,
            };
        }
    }

    false
}

fn proceed_replica_request_http(
    url: &str,
    http: &Option<MapServiceNodeHTTP>,
    metrics: &Option<MapMetrics>,
) -> bool {
    debug!("prober poll will fire for http target: {}", &url);

    // Unpack HTTP body match
    let http_body_healthy_match = http.as_ref().and_then(|ref http_inner| {
        if let Some(ref http_inner_body_inner) = http_inner.body {
            if let Some(ref healthy_match_inner) = http_inner_body_inner.healthy_match {
                if healthy_match_inner.is_empty() == false {
                    return Some(healthy_match_inner.to_owned());
                }
            }
        }

        None
    });

    // Unpack dead timeout
    let dead_timeout = acquire_dead_timeout(metrics);

    // Acquire replica response
    let mut response_body = Vec::new();

    let response = Request::new(&Uri::from_str(&url).expect("invalid replica request uri"))
        .connect_timeout(Some(dead_timeout))
        .read_timeout(Some(dead_timeout))
        .write_timeout(Some(dead_timeout))
        .method(if http_body_healthy_match.is_some() == true {
            Method::GET
        } else {
            Method::HEAD
        })
        .header("User-Agent", &*POLL_HTTP_HEADER_USERAGENT)
        .send(&mut response_body);

    // Handle response
    if let Ok(response) = response {
        let status_code = u16::from(response.status_code());

        debug!(
            "prober poll result received for url: {} with status: {}",
            &url, status_code
        );

        // Unpack HTTP status codes
        let mut http_healthy_above = NODE_HTTP_HEALTHY_ABOVE;
        let mut http_healthy_below = NODE_HTTP_HEALTHY_BELOW;

        if let &Some(ref http_inner) = http {
            if let Some(ref http_inner_status_inner) = http_inner.status {
                if let Some(healthy_above_inner) = http_inner_status_inner.healthy_above {
                    http_healthy_above = healthy_above_inner;
                }

                if let Some(healthy_below_inner) = http_inner_status_inner.healthy_below {
                    http_healthy_below = healthy_below_inner;
                }
            }
        }

        // Consider as UP?
        if status_code >= http_healthy_above && status_code < http_healthy_below {
            // Check response body for match? (if configured)
            if let Some(ref http_body_healthy_match_inner) = http_body_healthy_match {
                if !response_body.is_empty() {
                    debug!(
                        "checking prober poll result response text for url: {} for any match",
                        &url
                    );

                    // Check transfer encoding of response body
                    let transfer_encoding = response
                        .headers()
                        .get("Transfer-Encoding")
                        .map(|value| value.to_owned())
                        .unwrap_or("identity".to_string());

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

                    // Doesnt match? Consider as DOWN.
                    let text_search = TwoWaySearcher::new(http_body_healthy_match_inner.as_bytes())
                        .search_in(&response_body);

                    if text_search.is_none() {
                        return false;
                    }
                } else {
                    debug!("could not unpack response text for url: {}", &url);

                    // Consider as DOWN (the response text could not be checked)
                    return false;
                }
            }

            return true;
        }
    } else {
        debug!("prober poll result was not received for url: {}", &url);
    }

    // Consider as DOWN.
    false
}

fn acquire_dead_timeout(metrics: &Option<MapMetrics>) -> Duration {
    Duration::from_secs(if let &Some(ref metrics_inner) = metrics {
        metrics_inner.local.delay_dead
    } else {
        20
    })
}
