FROM rust:buster as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/discord_bot ./bot

ENTRYPOINT ["./bot", "start"]
