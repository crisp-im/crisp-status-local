Packaging
=========

This file contains quick reminders and notes on how to package `crisp-status-local`.

We consider here the packaging flow of `crisp-status-local` version `1.0.0`, for target architecture `i686` and distribution `debian9` (the steps are alike for `x86_64`):

1. **How to bump `crisp-status-local` version before a release:**
    1. Bump version in `Cargo.toml` to `1.0.0`
    2. Execute `cargo update` to bump `Cargo.lock`

2. **How to build `crisp-status-local` for Linux on Debian:**
    1. `apt-get install -y git build-essential pkg-config`
    2. `curl https://sh.rustup.rs -sSf | sh` (install the `nightly` toolchain)
    3. `git clone https://github.com/crisp-im/crisp-status-local.git`
    4. `cd crisp-status-local/`
    5. `cargo build --release`

3. **How to package built binary and release it on GitHub:**
    1. `mkdir crisp-status-local`
    2. `mv target/release/crisp-status-local crisp-status-local/`
    3. `strip crisp-status-local/crisp-status-local`
    4. `cp -r config.cfg res crisp-status-local/`
    5. `tar -czvf v1.0.0-i686-debian9.tar.gz crisp-status-local`
    6. `rm -r crisp-status-local/`
    7. Publish the archive on the [releases](https://github.com/crisp-im/crisp-status-local/releases) page on GitHub

4. **How to update other repositories:**
    1. Publish package on Crates: `cargo publish`
