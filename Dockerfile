FROM rust:1.79 AS builder
WORKDIR /app
COPY . .
# build in release
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/bitespeed_identity ./
COPY .env ./
ENV RUST_LOG=info
CMD ["./bitespeed_identity"]
