FROM rust:1.85-slim-bullseye as builder

WORKDIR /service/web-server

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

# Install minimal runtime dependencies if needed
# RUN apt-get update && apt-get install -y --no-install-recommends \
#     libssl-dev \
#     ca-certificates \
#     && rm -rf /var/lib/apt/lists/*

WORKDIR /service/web-server

COPY --from=builder /service/web-server/target/release/web-server ./bin/
COPY --from=builder /service/web-server/entrypoint.sh ./
COPY --from=builder /service/web-server/restart.sh ./

RUN chmod +x entrypoint.sh restart.sh

ENTRYPOINT [ "/bin/bash", "-c", "source ~/.bashrc && /service/web-server/entrypoint.sh ${@}", "--" ]
