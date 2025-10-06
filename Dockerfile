FROM rust:latest AS builder

WORKDIR /waygate

COPY Cargo.toml Cargo.lock ./
COPY message ./message
COPY server ./server
COPY wire ./wire
COPY generate-keys ./generate-keys

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/waygate/target \
    cargo build --release -p server

RUN mkdir -p /waygate/artifacts

RUN --mount=type=cache,target=/waygate/target \
    find ./target/release \( -name server -o -name libsteam_api.so \) -exec cp {} /waygate/artifacts/ \;

RUN mkdir -p /waygate/libs/lib64 /waygate/libs/lib/x86_64-linux-gnu && \
    cp /lib/x86_64-linux-gnu/libgcc_s.so.1 /waygate/libs/lib/x86_64-linux-gnu/ && \
    cp /lib/x86_64-linux-gnu/libm.so.6 /waygate/libs/lib/x86_64-linux-gnu/ && \
    cp /lib/x86_64-linux-gnu/libc.so.6 /waygate/libs/lib/x86_64-linux-gnu/ && \
    cp /lib/x86_64-linux-gnu/libdl.so.2 /waygate/libs/lib/x86_64-linux-gnu/ && \
    cp /lib/x86_64-linux-gnu/libpthread.so.0 /waygate/libs/lib/x86_64-linux-gnu/ && \
    cp /lib/x86_64-linux-gnu/librt.so.1 /waygate/libs/lib/x86_64-linux-gnu/ && \
    cp /lib64/ld-linux-x86-64.so.2 /waygate/libs/lib64/




FROM debian:bookworm-slim AS steamcmd_stage

WORKDIR /steam

RUN apt-get update && \
    apt-get install -y --no-install-recommends --no-install-suggests \
    lib32stdc++6 \
    ca-certificates \
    curl && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir -p /steam && \
    curl -fsSL 'https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz' | tar xvzf - -C /steam && \
    ./steamcmd.sh +quit




FROM scratch

WORKDIR /waygate

COPY --from=builder /waygate/artifacts/ ./
COPY --from=builder /waygate/libs/lib /lib/
COPY --from=builder /waygate/libs/lib64 /lib64/

COPY --from=steamcmd_stage /steam/linux64/steamclient.so /waygate/.steam/sdk64/steamclient.so

COPY steam_appid.txt /waygate/steam_appid.txt

ENV LD_LIBRARY_PATH=/waygate:/waygate/.steam/linux64
ENV PATH=/waygate:/waygate/steam:$PATH
ENV HOME=/waygate

ENV STEAMROOT=/waygate/.steam
ENV STEAMCLIENT_LIBRARY_PATH=/waygate/.steam/linux64
ENV STEAM_RUNTIME_LIBRARY_PATH=/waygate/.steam/linux64

CMD ["/waygate/server"]
