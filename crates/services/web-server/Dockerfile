FROM rust:1.85-slim-bullseye AS builder

WORKDIR /build

COPY ../../../ .

RUN cargo build -p web-server --release

FROM debian:bullseye-slim

WORKDIR /service

COPY --from=builder /build/target/release/web-server ./web-server

COPY --from=builder /build/web-folder ./web-folder/
COPY --from=builder /build/sql ./sql/

COPY --from=builder /build/crates/services/web-server/entrypoint.sh ./entrypoint.sh

RUN chmod +x entrypoint.sh

ENTRYPOINT [ "/bin/bash", "-c", "source ~/.bashrc && /service/entrypoint.sh ${@}", "--" ]
