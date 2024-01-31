# Use the official Rust image as the base image
FROM rust:1.54

# Set the working directory
WORKDIR /usr/src/app

# Copy over your source code
COPY . .

# Build the application
RUN cargo build --release

# Set the command to run your application
CMD DISCORD_TOKEN=$DISCORD_TOKEN cargo run