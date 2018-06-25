// crisp-status-local
//
// Crisp Status local probe relay
// Copyright: 2018, Crisp IM SARL
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::fmt;

use url::Url;
use serde::{Deserialize, Deserializer};
use serde::de::{Error, Visitor};

#[derive(Serialize, Debug, Clone)]
pub enum ReplicaURL {
    TCP(String, String, u16),
    HTTP(String, String),
    HTTPS(String, String),
}

impl ReplicaURL {
    pub fn parse_from(raw_url: &str) -> Result<ReplicaURL, ()> {
        match Url::parse(raw_url) {
            Ok(parsed_url) => {
                match parsed_url.scheme() {
                    "tcp" => {
                        match (parsed_url.host_str(), parsed_url.port()) {
                            (Some(host), Some(port)) => {
                                Ok(ReplicaURL::TCP(raw_url.to_owned(), host.to_string(), port))
                            }
                            _ => Err(()),
                        }
                    }
                    "http" => Ok(ReplicaURL::HTTP(
                        raw_url.to_owned(),
                        parsed_url.into_string(),
                    )),
                    "https" => Ok(ReplicaURL::HTTPS(
                        raw_url.to_owned(),
                        parsed_url.into_string(),
                    )),
                    _ => Err(()),
                }
            }
            _ => Err(()),
        }
    }

    pub fn get_raw(&self) -> &str {
        match self {
            &ReplicaURL::TCP(ref raw_url, _, _) => raw_url,
            &ReplicaURL::HTTP(ref raw_url, _) => raw_url,
            &ReplicaURL::HTTPS(ref raw_url, _) => raw_url,
        }
    }
}

impl<'de> Deserialize<'de> for ReplicaURL {
    fn deserialize<D>(de: D) -> Result<ReplicaURL, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReplicaURLVisitor;

        impl<'de> Visitor<'de> for ReplicaURLVisitor {
            type Value = ReplicaURL;

            fn expecting(&self, format: &mut fmt::Formatter) -> fmt::Result {
                format.write_str("a TCP, HTTP or HTTPS url")
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<ReplicaURL, E> {
                ReplicaURL::parse_from(value).map_err(|_| E::custom("invalid"))
            }
        }

        de.deserialize_str(ReplicaURLVisitor)
    }
}
