FROM rust:1.69 as builder
RUN USER=root cargo new --bin hubify-controller
RUN mkdir /common
WORKDIR /hubify-controller
COPY ./Cargo.toml ./Cargo.toml
# COPY ../common /common 
RUN apt-get update && apt-get install -y build-essential protobuf-compiler
RUN cargo build --release
RUN rm -rf ./src
COPY ./src ./src
RUN rm -f ./target/release/hubify-controller*
RUN cargo build --release


# ---- RUNTIME ----
FROM debian:buster-slim as runtime
ARG APP=/usr/src/app
RUN apt-get update \
  && apt-get install -y ca-certificates tzdata \
  && rm -rf /var/lib/apt/lists/*
ENV TZ=Etc/UTC \
  APP_USER=appuser
RUN adduser --disabled-password --gecos "" --home "$APP" "$APP_USER"
WORKDIR $APP
COPY --from=builder /hubify-controller/target/release/hubify-controller $APP/hubify-controller
RUN chown -R $APP_USER:$APP_USER $APP
USER $APP_USER
EXPOSE 3000
CMD ["./hubify-controller"]

# ---- DEV ----
FROM rust:1.69 as dev
RUN cargo new --bin hubify-controller
WORKDIR /hubify-controller
COPY crates/controller/Cargo.toml ./Cargo.toml
COPY crates/common /common 
RUN apt-get update && apt-get install -y build-essential protobuf-compiler
RUN cargo build
RUN cargo install cargo-watch
RUN rm -rf ./src
COPY crates/controller/src ./src
RUN rm -f ./target/debug/hubify-controller*
CMD ["cargo","watch", "-x","run --no-default-features"]
