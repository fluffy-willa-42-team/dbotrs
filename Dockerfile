# First stage: Build the application
FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# Second stage: Run the application
FROM debian:buster-slim

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/dbotrs .

CMD DISCORD_TOKEN=$DISCORD_TOKEN ./dbotrs