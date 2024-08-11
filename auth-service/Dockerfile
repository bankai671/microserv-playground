FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM ubuntu:latest

RUN apt-get update && apt-get install -y libc6 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/auth-service .

EXPOSE 8001

CMD ["./auth-service"]
