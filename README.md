crisp-status-local
==================

[![Build Status](https://travis-ci.org/crisp-im/crisp-status-local.svg?branch=master)](https://travis-ci.org/crisp-im/crisp-status-local) [![Dependency Status](https://deps.rs/repo/github/crisp-im/crisp-status-local/status.svg)](https://deps.rs/repo/github/crisp-im/crisp-status-local)

**Crisp Status Local is used to monitor internal hosts and report their status to Crisp Status.**

Crisp Status Local is a daemon that you can use to report internal service health to your Crisp Status-powered status page. It is designed to be used behind a firewall, and to monitor hosts bound to your local loop or LAN network (eg. a MySQL server running on a LAN IP address).

Install Crisp Status Local on a server of yours and configure it with your Crisp Status token; it will then automatically start monitoring all nodes that you configured in Crisp in `local` mode. The `local` mode is similar to `poll` mode, but is used specifically for `crisp-status-local` monitoring.

Copyright 2018 Crisp IM SARL. See LICENSE for copying information.

* **😘 Maintainer**: [@valeriansaliou](https://github.com/valeriansaliou)

## What is Crisp Status?

[Crisp Status](https://crisp.chat/en/status/) is a status page service available on [Crisp](https://crisp.chat/en/). See a live demo of Crisp Status on [Enrich Status Page](https://status.enrichdata.com/) (Enrich is a service that uses Crisp).

It lets Crisp users monitor their critical systems using a variety of methods: `push` for applications using a Crisp Status Reporter library, `poll` for public Internet-wide HTTP & TCP services, and `local` for private LAN-wide HTTP & TCP services.

Crisp Status alerts the monitored website operators when a node goes down. If it stays down for too long, website users are also notified via an alert on the Crisp Chatbox, Crisp Helpdesk, as well as access to details on the Crisp Status Page.

Crisp Status Local lets Crisp Status users monitor nodes that are configured in `local` mode (ie. private LAN-wide HTTP & TCP services), aside other monitoring methods that do not require the Crisp Status Local utility.

## How does it work?

Crisp Status Local is to be installed on a server in your infrastructure. A maximum of one Crisp Status Local instance can run per Crisp Status Page.

Crisp Status Local dynamically pulls your Crisp Status configuration and checks for `local` mode nodes health. It then reports whether those internal nodes are `healthy`, `sick` or `dead`.

**👉 Crisp Status Local is open-source and built in Rust; thus you are free to review, modify its code and compile it yourself. We know that running such a binary in your infrastructure can be sensitive, that's why we made it open-source.**

## How to use it?

### Installation

#### A. Install from packages (⭐️ recommended)

Crisp Status Local provides [pre-built packages](https://packagecloud.io/crisp-im/crisp-status-local) for Debian-based systems (Debian, Ubuntu, etc.).

**Important: Crisp Status Local only provides Debian 8 64 bits packages for now (Debian Jessie). You will still be able to use them on other Debian versions, as well as Ubuntu.**

**1️⃣ Add the Crisp Status Local APT repository (eg. for Debian Jessie):**

```bash
echo "deb https://packagecloud.io/crisp-im/crisp-status-local/debian/ jessie main" > /etc/apt/sources.list.d/crisp-im_crisp-status-local.list
curl -L https://packagecloud.io/crisp-im/crisp-status-local/gpgkey 2> /dev/null | apt-key add - &>/dev/null
apt-get update
```

**2️⃣ Install the Crisp Status Local package:**

```bash
apt-get install crisp-status-local
```

**3️⃣ Edit the pre-filled Crisp Status Local configuration file:**

```bash
nano /etc/crisp-status-local.cfg
```

**4️⃣ Restart Crisp Status Local:**

```
service crisp-status-local restart
```

#### B. Install from Docker Hub

You might find it convenient to run Crisp Status Local via Docker. You can find the pre-built Crisp Status Local image on Docker Hub as [crisp-im/crisp-status-local](https://hub.docker.com/r/crisp-im/crisp-status-local/).

First, pull the `crisp-im/crisp-status-local` image:

```bash
docker pull crisp-im/crisp-status-local:v1.0.0
```

Then, seed it a configuration file and run it (replace `/path/to/your/crisp-status-local/config.cfg` with the path to your configuration file):

```bash
docker run -v /path/to/your/crisp-status-local/config.cfg:/etc/crisp-status-local.cfg crisp-im/crisp-status-local:v1.0.0
```

In the configuration file, ensure that:

* `report.token` is set to your Crisp Status Reporter token (you can get it on your Crisp dashboard)

#### C. Install from releases

You can install Crisp Status Local by pulling the latest release from the [Crisp Status Local releases](https://github.com/crisp-im/crisp-status-local/releases) page.

Make sure to pick the correct server architecture.

#### D. Install from Cargo

If you prefer managing `crisp-status-local` via Rust's Cargo, install it directly via `cargo install`:

```bash
cargo install crisp-status-local
```

Ensure that your `$PATH` is properly configured to source the Crates binaries, and then run Crisp Status Local using the `crisp-status-local` command.

#### E. Install from source

The last option is to pull the source code from Git and compile Crisp Status Local via `cargo`:

```bash
cargo build --release
```

You can find the built binaries in the `./target/release` directory.

### Configuration

Use the sample [config.cfg](https://github.com/crisp-im/crisp-status-local/blob/master/config.cfg) configuration file and adjust it to your own environment.

**Available configuration options are commented below, with allowed values:**

**[server]**

* `log_level` (type: _string_, allowed: `debug`, `info`, `warn`, `error`, default: `warn`) — Verbosity of logging, set it to `error` in production

**[report]**

* `token` (type: _string_, allowed: any string, no default) — Your Crisp Status Reporter token (you can get it on your Crisp dashboard)

**Notice: if the `report.token` value is invalid, you will see errors in your `syslog` when the daemon is running.**

### Run

If you installed Crisp Status Local from packages (which is recommended), you can start Crisp Status Local as such:

`service crisp-status-local start`

Otherwise, Crisp Status Local can be run as such:

`./vigil -c /path/to/config.cfg`
