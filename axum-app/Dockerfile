# Build stage
FROM rust:1.75-slim AS builder

RUN apt-get update && apt-get install -y \
    openssl libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Optimize layer caching: copy Cargo.* first
COPY Cargo.toml Cargo.lock* ./
RUN mkdir -p src && echo 'fn main() {}' > src/lib.rs
RUN cargo build --release 2>/dev/null || true

# Now copy full source (except .dockerignore'd files)
COPY . .

# Build release binary
RUN cargo build --release --bin axum-app

# Final minimal runtime image (distroless doesn't have libssl → use slim Debian)
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 1000 appuser

WORKDIR /app

# Copy binaries and migrations
COPY --from=builder /app/target/release/axum-app /app/axum-app
COPY --from=builder /app/migrations /app/migrations

# Set permissions
RUN chown -R appuser:appuser /app && \
    chmod +x /app/axum-app

USER appuser
EXPOSE 8000
CMD ["./axum-app"]
