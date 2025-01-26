# ============================
# 1) Build Stage
# ============================
FROM rust:1.84.0-bookworm AS builder

WORKDIR /app

# Copy your workspace root's files
COPY Cargo.toml Cargo.lock ./
COPY services services

# Bookworm dev libs (for building with OpenSSL)
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Build the 'compute' crate in release mode
RUN cargo build --release -p compute

# ============================
# 2) Final Stage
# ============================
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime libraries: glibc is included by default, but ensure libssl3 for OpenSSL 3
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /app/target/release/compute /usr/local/bin/compute

EXPOSE 8080
CMD ["compute"]
