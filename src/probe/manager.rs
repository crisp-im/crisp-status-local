// crisp-status-local
//
// Crisp Status local probe relay
// Copyright: 2018, Crisp IM SAS
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::thread;
use std::time::Duration;

use super::map::{acquire as map_acquire, Map};
use super::poll::dispatch as poll_dispatch;

const PROBE_RUN_HOLD_SECONDS: u64 = 2;
const PROBE_CHECK_INTERVAL_SECONDS: u64 = 120;

pub fn run() {
    // Initialize map
    let mut map = Map {
        date: None,
        metrics: None,
        services: Vec::new(),
    };

    // Hold on a bit before first cycle
    thread::sleep(Duration::from_secs(PROBE_RUN_HOLD_SECONDS));

    debug!("will run first probe cycle");

    // Start cycling
    loop {
        cycle(&mut map);

        info!(
            "done cycling probe, holding for next cycle: {}s",
            PROBE_CHECK_INTERVAL_SECONDS
        );

        // Hold on a bit for next cycle
        thread::sleep(Duration::from_secs(PROBE_CHECK_INTERVAL_SECONDS));

        debug!("holding for next probe cycle, will run next cycle");
    }
}

fn cycle(map: &mut Map) {
    // Acquire map changes
    match map_acquire(map) {
        Ok(_) => {
            debug!("acquired map for probe cycle");

            // Dispatch polls
            poll_dispatch(&map.services, &map.metrics, PROBE_CHECK_INTERVAL_SECONDS);
        }
        Err(err) => {
            warn!("probe cycle error in map acquire: {:?}", err);
        }
    }
}
