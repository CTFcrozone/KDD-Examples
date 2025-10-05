FROM rust:1.85-slim-bullseye AS rs-builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/

RUN cargo fetch
RUN cargo build --release --workspace

CMD mkdir -p /output && \
    find /app/target/release -maxdepth 1 -type f -executable \
    -not -name '*.d' -not -name '*.so' -not -name '*.rlib' \
    -exec cp {} /output/ \; && \
    chmod -R 755 /output && \
    chown -R 1000:1000 /output
