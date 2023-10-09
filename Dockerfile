FROM --platform=x86_64 rust:1-slim AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev

WORKDIR /home

COPY Cargo.toml .
COPY Cargo.lock .
COPY src src
RUN cargo install --path . --features "sqlx-cli"

RUN  --mount=type=cache,target=/home/.cargo cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /home/target/x86_64-unknown-linux-musl/release/rust /app
COPY --from=builder /usr/local/cargo/bin/sqlx /sqlx
# CMD ["/app"]
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["entrypoint.sh"]