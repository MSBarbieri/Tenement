FROM rust:1.69 as builder
RUN USER=root cargo new --bin tenement-controller
WORKDIR /tenement-controller
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN apt-get update && apt-get install -y build-essential protobuf-compiler
RUN cargo build --release
RUN rm -rf ./src
COPY ./src ./src
RUN rm -f ./target/release/tenement-controller*
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
COPY --from=builder /tenement-controller/target/release/tenement-controller $APP/tenement-controller
RUN chown -R $APP_USER:$APP_USER $APP
USER $APP_USER
EXPOSE 3000
CMD ["./tenement-controller"]

# ---- DEV ----
FROM rust:1.69 as dev
RUN USER=root cargo new --bin tenement-controller
WORKDIR /tenement-controller
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN apt-get update && apt-get install -y build-essential protobuf-compiler
RUN cargo build
RUN cargo install cargo-watch
RUN rm -rf ./src
COPY ./src ./src
RUN rm -f ./target/debug/tenement-controller*
CMD ["cargo","watch", "-x","run --no-default-features"]
