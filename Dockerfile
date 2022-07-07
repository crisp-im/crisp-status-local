FROM rustlang/rust:nightly-buster-slim AS build

RUN apt-get update
RUN apt-get install -y musl-tools

RUN rustup --version
RUN rustup target add x86_64-unknown-linux-musl

RUN rustc --version && \
    rustup --version && \
    cargo --version

WORKDIR /app
COPY . /app
RUN cargo clean && cargo build --release --target x86_64-unknown-linux-musl
RUN strip ./target/x86_64-unknown-linux-musl/release/crisp-status-local

FROM scratch

WORKDIR /usr/src/crisp-status-local

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/crisp-status-local /usr/local/bin/crisp-status-local

CMD [ "crisp-status-local", "-c", "/etc/crisp-status-local.cfg" ]
