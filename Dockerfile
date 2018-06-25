FROM rustlang/rust:nightly

WORKDIR /usr/src/crisp-status-local
COPY ./res/assets/ ./res/assets/

RUN cargo install crisp-status-local
CMD [ "crisp-status-local", "-c", "/etc/crisp-status-local.cfg" ]
