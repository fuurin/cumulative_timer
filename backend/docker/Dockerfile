FROM rust:latest
ENV APP_HOME=/backend \
    USER=fuurin-noimin \
    CARGO_TARGET_DIR=/tmp/target
RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres
WORKDIR $APP_HOME
