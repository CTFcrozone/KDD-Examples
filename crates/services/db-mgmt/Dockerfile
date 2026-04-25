# Stage 1: Builder
FROM rust:1.94-slim-bullseye AS builder

WORKDIR /build

# Copy everything (or just what you need)
COPY . .

# Build only the web-server binary
RUN cargo build -p db-mgmt --release

# Stage 2: Runtime
FROM debian:bullseye-slim

WORKDIR /service

# Copy the compiled binary
COPY --from=builder /build/target/release/db-mgmt ./db-mgmt

# Copy web folder and SQL folder
COPY --from=builder /build/crates/services/db-mgmt/sql ./sql/

# Copy entrypoint script
COPY --from=builder /build/crates/services/db-mgmt/entrypoint.sh ./entrypoint.sh

# Make entrypoint executable
RUN chmod +x entrypoint.sh

# Optional: minimal runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
	ca-certificates \
	libssl-dev \
	&& rm -rf /var/lib/apt/lists/*

# Entrypoint
ENTRYPOINT ["/bin/bash", "-c", "source ~/.bashrc && /service/entrypoint.sh ${@}", "--"]
