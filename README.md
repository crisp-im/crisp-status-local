Crisp Status Local
==================

[![Test and Build](https://github.com/crisp-im/crisp-status-local/workflows/Test%20and%20Build/badge.svg?branch=master)](https://github.com/crisp-im/crisp-status-local/actions?query=workflow%3A%22Test+and+Build%22)

**Crisp Status Local is used to monitor internal hosts and report their status to Crisp Status.**

Crisp Status Local is a daemon that you can use to report internal service health to your Crisp Status-powered status page. It is designed to be used behind a firewall, and to monitor hosts bound to your local loop or LAN network (eg. a MySQL server running on a LAN IP address).

Install Crisp Status Local on a server of yours and configure it with your Crisp Status token; it will then automatically start monitoring all nodes that you configured in Crisp in `local` mode. The `local` mode is similar to `poll` mode, but is used specifically for `crisp-status-local` monitoring.

Copyright 2018 Crisp IM SAS. See LICENSE for copying information.

_Tested at Rust version: `rustc 1.40.0 (73528e339 2019-12-16)`_

* **😘 Maintainer**: [@valeriansaliou](https://github.com/valeriansaliou)

## What is Crisp Status?

[Crisp Status](https://crisp.chat/en/status/) is a status page service available on [Crisp](https://crisp.chat/en/). See a live demo of Crisp Status on [Enrich Status Page](https://status.enrich.email/) (Enrich is a service that uses Crisp).

It lets Crisp users monitor their critical systems using a variety of methods: `push` for applications using a Crisp Status Reporter library, `poll` for public Internet-wide HTTP & TCP services, and `local` for private LAN-wide HTTP & TCP services.

Crisp Status alerts the monitored website operators when a node goes down. If it stays down for too long, website users are also notified via an alert on the Crisp Chatbox, Crisp Helpdesk, as well as access to details on the Crisp Status Page.

Crisp Status Local lets Crisp Status users monitor nodes that are configured in `local` mode (ie. private LAN-wide HTTP & TCP services), aside other monitoring methods that do not require the Crisp Status Local utility.

## How does it work?

Crisp Status Local is to be installed on a server in your infrastructure. A maximum of one Crisp Status Local instance can run per Crisp Status Page.

Crisp Status Local dynamically pulls your Crisp Status configuration and checks for `local` mode nodes health. It then reports whether those internal nodes are `healthy`, `sick` or `dead`.

**👉 Crisp Status Local is open-source and built in Rust; thus you are free to review, modify its code and compile it yourself. We know that running such a binary in your infrastructure can be sensitive, that's why we made it open-source.**

## How to add monitored nodes?

You can easily add local nodes to be monitored on your Crisp dashboard, as follows:

<p align="center">
  <img width="605" src="https://crisp-im.github.io/crisp-status-local/images/setup.gif" alt="How to add monitored nodes">
</p>

## How to use it?

### Installation

#### A. Install from Docker Hub (⭐️ recommended)

You might find it convenient to run Crisp Status Local via Docker. You can find the pre-built Crisp Status Local image on Docker Hub as [crispim/crisp-status-local](https://hub.docker.com/r/crispim/crisp-status-local/).

First, pull the `crispim/crisp-status-local` image:

```bash
docker pull crispim/crisp-status-local:v1.3.0
```

Then, seed it a configuration file and run it (replace `/path/to/your/crisp-status-local/config.cfg` with the path to your configuration file):

```bash
docker run -v /path/to/your/crisp-status-local/config.cfg:/etc/crisp-status-local.cfg crispim/crisp-status-local:v1.3.0
```

In the configuration file, ensure that:

* `report.token` is set to your Crisp Status Reporter token (you can get it on your Crisp dashboard)

#### B. Install from binary

A pre-built binary of Crisp Status Local is shared in the releases on GitHub. You can simply download the latest binary version from the [releases page](https://github.com/crisp-im/crisp-status-local/releases), and run it on your server. Each release binary comes with an `.asc` signature file, which can be verified using [Crisp GPG key](https://docs.crisp.chat/guides/others/security-practices/#vulnerability-disclosures).

You will still need to provide the binary with the configuration file, so make sure you have a Crisp Status Local `config.cfg` file ready somewhere.

_The binary provided is statically-linked, which means that it will be able to run on any Linux-based server. Still, it will not work on MacOS or Windows machines._

#### C. Install from Cargo

If you prefer managing `crisp-status-local` via Rust's Cargo, install it directly via `cargo install`:

```bash
cargo install crisp-status-local
```

Ensure that your `$PATH` is properly configured to source the Crates binaries, and then run Crisp Status Local using the `crisp-status-local` command.

#### D. Build from source

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

Crisp Status Local can be run as such:

`./crisp-status-local -c /path/to/config.cfg`

## Get more help

You can find more help on our helpdesk article: [How to setup the Crisp Status Local service?](https://help.crisp.chat/en/article/1vbyqkt/)

## :fire: Report A Vulnerability

If you find a vulnerability in Crisp Status Local, you are more than welcome to report it directly to [@crisp-im](https://github.com/crisp-im) by sending an encrypted email to [security@crisp.chat](mailto:security@crisp.chat). Do not report vulnerabilities in public GitHub issues, as they may be exploited by malicious people to target production servers running an unpatched Crisp Status Local server.

**:warning: You must encrypt your email using [@crisp-im](https://github.com/crisp-im) GPG public key available at: [Vulnerability Disclosures](https://docs.crisp.chat/guides/others/security-practices/#vulnerability-disclosures).**
