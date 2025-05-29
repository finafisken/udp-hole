###############
### BUILDER ###
###############
FROM rust:1.87.0-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src

RUN cargo build --release

###############
### RUNTIME ###
###############
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/udp-hole ./

WORKDIR /app

# UDP port 3400
EXPOSE 3400/udp

CMD ["/app/udp-hole"]