FROM rust:1.79.0-slim-bullseye

WORKDIR /usr/src/app

RUN apt -y update && apt -y install libsodium-dev

COPY ./server ./server/
COPY ./fnrpc ./fnrpc/
COPY ./config ./config/
COPY ./Cargo.toml .

RUN ls

RUN cargo build --release

EXPOSE 10901

CMD ["./waygate-server"]
