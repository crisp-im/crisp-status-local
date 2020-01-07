FROM rustlang/rust:nightly AS build

RUN cargo install crisp-status-local

FROM debian:stretch-slim

WORKDIR /usr/src/crisp-status-local

COPY --from=build /usr/local/cargo/bin/crisp-status-local /usr/local/bin/crisp-status-local

RUN apt-get update
RUN apt-get install -y libssl-dev

CMD [ "crisp-status-local", "-c", "/etc/crisp-status-local.cfg" ]
