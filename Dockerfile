FROM rustlang/rust:nightly

WORKDIR /usr/src/crisp-status-local

RUN cargo install crisp-status-local
CMD [ "crisp-status-local", "-c", "/etc/crisp-status-local.cfg" ]
