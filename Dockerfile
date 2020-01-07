FROM rustlang/rust:nightly AS build

RUN apt-get update
RUN apt-get install -y musl-tools

RUN rustup --version
RUN rustup install nightly-2020-01-02 && \
    rustup default nightly-2020-01-02 && \
    rustup target add x86_64-unknown-linux-musl

RUN rustc --version && \
    rustup --version && \
    cargo --version

WORKDIR /app
COPY . /app
RUN cargo clean && cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

WORKDIR /usr/src/crisp-status-local

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/crisp-status-local /usr/local/bin/crisp-status-local

CMD [ "crisp-status-local", "-c", "/etc/crisp-status-local.cfg" ]
