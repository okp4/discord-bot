#--- Build stage
FROM clux/muslrust:1.64.0-stable as builder

WORKDIR /app

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

#--- Image stage
FROM alpine:3.16.2

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/discord_bot ./bot

ENTRYPOINT ["./bot"]
