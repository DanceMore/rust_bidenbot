# Use the official Rust image as the base image
FROM rust:1.74 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --package rust_bidenbot

### stage 2
# Create a new lightweight image with just the binary
FROM debian:bookworm-slim

# Set the working directory inside the container
WORKDIR /app

# Copy the binary from the builder stage to the final image
COPY --from=builder /app/target/release/rust_bidenbot /app/rust_bidenbot

# Command to run your Rocket application
CMD ["/app/rust_bidenbot"]
