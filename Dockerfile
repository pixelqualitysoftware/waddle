FROM rust:alpine AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM alpine:latest

COPY --from=builder /app/target/release/waddle /usr/local/bin/waddle

CMD ["waddle"]
