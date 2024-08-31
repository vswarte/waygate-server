FROM rust:latest AS builder

WORKDIR /waygate

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN cargo build --release

RUN mkdir /waygate/artifacts
RUN find ./target/release \( -name waygate-server -o -name waygate-generate-keys -o -name libsteam_api.so \) -exec cp {} /waygate/artifacts/ \;


FROM steamcmd/steamcmd:latest

WORKDIR /waygate

ENV LD_LIBRARY_PATH=/root/.local/share/Steam/steamcmd/linux64:/waygate

COPY --from=builder /waygate/artifacts/ ./
COPY announcements.toml logging.toml steam_appid.txt ./

COPY docker-entrypoint.sh ./
RUN chmod +x docker-entrypoint.sh
ENTRYPOINT ["./docker-entrypoint.sh"]
