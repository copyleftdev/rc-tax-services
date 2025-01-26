# ============================
# 1) Build Stage
# ============================
FROM rust:1.84.0-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY services services

RUN apt-get update && apt-get install -y pkg-config libssl-dev

RUN cargo build --release -p ingest

# ============================
# 2) Final Stage
# ============================
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ingest /usr/local/bin/ingest

EXPOSE 3000
CMD ["ingest"]
