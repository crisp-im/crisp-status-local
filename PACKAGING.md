Packaging
=========

This file contains quick reminders and notes on how to package `crisp-status-local`.

We consider here the packaging flow of `crisp-status-local` version `1.0.0` for Linux, for target architecture `x86_64`:

1. **How to setup `rust-musl-builder` on MacOS:**
    1. Follow setup instructions from: [rust-musl-builder](https://github.com/emk/rust-musl-builder)
    2. Pull the nightly Docker image: `docker pull ekidd/rust-musl-builder:nightly`

2. **How to bump `crisp-status-local` version before a release:**
    1. Bump version in `Cargo.toml` to `1.0.0`
    2. Execute `cargo update` to bump `Cargo.lock`

3. **How to build `crisp-status-local` for Linux on MacOS:**
    1. `rust-musl-builder-nightly cargo build --target=x86_64-unknown-linux-musl --release`
    2. `rust-musl-builder-nightly strip ./target/x86_64-unknown-linux-musl/release/crisp-status-local`

4. **How to package built binary and release it on GitHub:**
    1. `mkdir crisp-status-local`
    2. `mv target/x86_64-unknown-linux-musl/release/crisp-status-local crisp-status-local/`
    4. `cp config.cfg crisp-status-local/`
    5. `tar -czvf v1.0.0-x86_64.tar.gz crisp-status-local`
    6. `rm -r crisp-status-local/`
    7. Publish the archive on the [releases](https://github.com/crisp-im/crisp-status-local/releases) page on GitHub

5. **How to update Crates:**
    1. Publish package on Crates: `cargo publish`

6. **How to update Docker:**
    1. `docker build .`
    2. `docker tag [DOCKER_IMAGE_ID] crispim/crisp-status-local:v1.0.0` (insert the built image identifier)
    3. `docker push crispim/crisp-status-local:v1.0.0`
