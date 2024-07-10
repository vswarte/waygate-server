FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY ./fnrpc ./fnrpc
COPY ./server ./server

RUN cargo fetch

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/steam_appid.txt .
COPY --from=builder /usr/src/app/target/release/waygate-server .

EXPOSE 10901

CMD ["./waygate-server"]
