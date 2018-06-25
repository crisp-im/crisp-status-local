FROM rustlang/rust:nightly-slim

WORKDIR /usr/src/crisp-status-local

RUN apt-get update
RUN apt-get install -y pkg-config libssl-dev
RUN cargo install crisp-status-local
CMD [ "crisp-status-local", "-c", "/etc/crisp-status-local.cfg" ]
