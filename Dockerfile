FROM rustlang/rust:nightly-slim AS build

RUN cargo install crisp-status-local

FROM debian:stretch-slim

WORKDIR /usr/src/crisp-status-local

COPY --from=build /usr/local/cargo/bin/crisp-status-local /usr/local/bin/crisp-status-local

CMD [ "crisp-status-local", "-c", "/etc/crisp-status-local.cfg" ]
