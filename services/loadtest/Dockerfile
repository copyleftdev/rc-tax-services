# ============================
# 1) Builder Stage
# ============================
FROM rust:1.84.0-bookworm AS builder

WORKDIR /app

# Copy workspace-level files
COPY Cargo.toml Cargo.lock ./
COPY services services

RUN apt-get update && apt-get install -y pkg-config libssl-dev

RUN cargo build --release -p loadtest

# ============================
# 2) Final Runtime Stage
# ============================
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/loadtest /usr/local/bin/loadtest

# Optionally set default environment variables
ENV CONCURRENCY=10
ENV REQUESTS_PER_TASK=100
ENV COMPUTE_URL=http://localhost:8080/api/compute

CMD ["loadtest"]
