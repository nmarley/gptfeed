# Build stage
FROM rust:1.93-bookworm AS builder

WORKDIR /app

# Copy package manifest and install deps (layer caching)
COPY Cargo.toml Cargo.lock ./

# Dummy file for cargo to download/compile dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo check --release

# Copy actual source
COPY src ./src

RUN /bin/pwd
RUN /bin/ls

# Build release binaries
RUN cargo build --release
RUN strip /app/target/release/gptfeed

# Runtime stage
FROM debian:12-slim
WORKDIR /app

# Install CA certificates for TLS
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binaries from builder
COPY --from=builder /app/target/release/gptfeed /usr/local/bin/

CMD ["gptfeed"]
