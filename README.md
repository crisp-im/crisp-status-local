crisp-status-local
==================

[![Build Status](https://travis-ci.org/crisp-im/crisp-status-local.svg?branch=master)](https://travis-ci.org/crisp-im/crisp-status-local) [![Dependency Status](https://deps.rs/repo/github/crisp-im/crisp-status-local/status.svg)](https://deps.rs/repo/github/crisp-im/crisp-status-local)

**Crisp Status Local is used to monitor internal hosts and report their status to Crisp Status.**

Crisp Status Local is a daemon that you can use to report internal service health to your Crisp Status-powered status page. It is designed to be used behind a firewall, and to monitor hosts bound to your local loop or LAN network (eg. a MySQL server running on a LAN IP address).

Install Crisp Status Local on a server of yours and configure it with your Crisp Status token; it will then automatically start monitoring all nodes that you configured in Crisp in `local` mode. The `local` mode is similar to `poll` mode, but is used specifically for `crisp-status-local` monitoring.

**👉 See a live demo of Crisp Status on [Enrich Status Page](https://status.enrichdata.com/).**

## What is Crisp Status?

[Crisp Status](https://crisp.chat/en/status/) is a status page service available on [Crisp](https://crisp.chat/en/).

It lets Crisp users monitor their critical systems using a variety of methods: `push` for applications using a Crisp Status Reporter library, `poll` for public Internet-wide HTTP & TCP services, and `local` for private LAN-wide HTTP & TCP services.

Crisp Status alerts the monitored website operators when a node goes down. If it stays down for too long, website users are also notified via an alert on the Crisp Chatbox, Crisp Helpdesk, as well as access to details on the Crisp Status Page.

Crisp Status Local lets Crisp Status users monitor nodes that are configured in `local` mode (ie. private LAN-wide HTTP & TCP services), aside other monitoring methods that do not require the Crisp Status Local utility.

## How does it work?

Crisp Status Local is to be installed on a server in your infrastructure. A maximum of one Crisp Status Local instance can run per Crisp Status Page.

Crisp Status Local dynamically pulls your Crisp Status configuration and checks for `local` mode nodes health. It then reports whether those internal nodes are `healthy`, `sick` or `dead`.

Crisp Status Local is open-source and built in Rust; thus you are free to review, modify its code and compile it yourself. We know that running such a binary in your infrastructure can be sensitive, so we made it open-source.

## How to use it?

### Installation

TODO: content

### Configuration

TODO: content

### Run

TODO: content
