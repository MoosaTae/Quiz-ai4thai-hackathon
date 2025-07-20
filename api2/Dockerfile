FROM rust:1.88.0 as builder

WORKDIR /app
COPY Cargo.toml .
COPY src/ src/
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/api2 .

EXPOSE 6002

CMD ["./api2"]