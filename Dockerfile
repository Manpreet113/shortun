# ---------------------------------------------------
# 1. THE BUILD STAGE ("The Factory")
# ---------------------------------------------------
FROM rust:latest as builder

# Create a new empty shell project
WORKDIR /usr/src/app

# Copy the dependency files first
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to trick Cargo into building just dependencies
# This caches your dependencies so you don't redownload them every time you change one line of code.
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Now remove the dummy build
RUN rm -f target/release/deps/shortun*

# Copy the ACTUAL source code
COPY . .

# Build the actual application
RUN cargo build --release

# ---------------------------------------------------
# 2. THE RUNTIME STAGE ("The Shipping Container")
# ---------------------------------------------------
FROM debian:bookworm-slim

# Install OpenSSL and CA certificates (Needed for HTTPS and Postgres)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Create a non-root user (Security Best Practice)
# We don't want our app running as 'root' inside the container.
RUN useradd -ms /bin/bash appuser
USER appuser
WORKDIR /home/appuser

# Copy the binary from the "builder" stage
COPY --from=builder /usr/src/app/target/release/shortun ./app

# Expose the port
EXPOSE 3000

# The command to run when the container starts
CMD ["./app"]