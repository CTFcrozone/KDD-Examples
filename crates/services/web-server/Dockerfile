FROM rust:1.85-slim-bullseye AS builder

WORKDIR /service

COPY ../../../ .

RUN cargo build -p web-server --release

FROM debian:bullseye-slim

WORKDIR /service

COPY --from=builder /service/target/release/web-server ./web-server
COPY --from=builder /service/services/web-server/web-folder ./web-folder/
COPY --from=builder /service/services/web-server/sql ./sql/

COPY --from=builder /service/web-server/entrypoint.sh ./

RUN chmod +x entrypoint.sh

ENTRYPOINT [ "/bin/bash", "-c", "source ~/.bashrc && /service/web-server/entrypoint.sh ${@}", "--" ]
