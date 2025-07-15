# Use the official Rust image as the base
FROM rust:1.75 AS builder

# Set the working directory in the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./
COPY compiler_core/Cargo.toml ./compiler_core/

# Create dummy source files to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN mkdir compiler_core/src && echo "pub fn test() {}" > compiler_core/src/lib.rs

# Build dependencies (this will be cached)
RUN cargo build --release

# Copy the actual source code
COPY src ./src
COPY compiler_core/src ./compiler_core/src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Set the working directory
WORKDIR /app

# Copy the built binary from builder stage
COPY --from=builder /app/target/release/compiler_project /app/

# Change ownership to appuser
RUN chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose the port
EXPOSE 3000

# Set environment variables
ENV RUST_LOG=info

# Run the application
CMD ["./compiler_project"] 