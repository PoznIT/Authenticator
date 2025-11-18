FROM rust:slim-bullseye AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN apt-get update -y && \
  apt-get install -y pkg-config make g++ libssl-dev && \
  rustup target add x86_64-unknown-linux-gnu
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /app/target/release/authenticator .
COPY ./config/config-prod.yml ./config.yml

EXPOSE 6767

CMD ["./authenticator", "./config.yml"]
