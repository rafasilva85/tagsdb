# Use a Rust image based on Bullseye to match the runtime GLIBC
FROM rust:1.78-bullseye as builder

# Set the working directory
WORKDIR /usr/src/uns_cli

# Copy the Cargo manifest and lock file
COPY Cargo.toml Cargo.lock ./

# Build dependencies first to leverage Docker cache
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a minimal image for the final stage (Bullseye has newer GLIBC)
FROM debian:bullseye-slim

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/uns_cli/target/release/uns_cli /usr/local/bin/uns_cli

# Set the command to run the application and redirect output to a file
# We'll override this in docker-compose.yml, but it's good practice to have a default
CMD ["/bin/bash", "-c", "uns_cli run > output.log 2>&1"]
