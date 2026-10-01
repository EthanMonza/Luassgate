# syntax=docker/dockerfile:1

# ---------- build ----------
# Pinned to the latest stable 1.x line (floating major tag):
# old pinned versions (e.g. 1.82) break when transitive deps start requiring edition2024 (Cargo 1.85+).
FROM rust:1-slim-bookworm AS builder
WORKDIR /app

# teloxide / reqwest (openssl) need C build dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# ---------- runtime ----------
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 10001 bot

WORKDIR /app
COPY --from=builder /app/target/release/telegram_bot_rust /app/bot
USER bot

ENV RUST_LOG=info

CMD ["/app/bot"]
