# Build stage
FROM rustlang/rust:nightly-alpine AS builder

# Install system dependencies for static linking
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

# Add musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Set working directory
WORKDIR /app

# Copy workspace configuration first
COPY Cargo.toml Cargo.lock ./

# Copy all source code
COPY aginsensors_core/ ./aginsensors_core/
COPY daemon/ ./daemon/
COPY database_influx/ ./database_influx/
COPY modules/ ./modules/
COPY connector_socketio/ ./connector_socketio/
COPY connector_mqtt/ ./connector_mqtt/
COPY modifier_template/ ./modifier_template/
COPY connector_modbus/ ./connector_modbus/

# Set environment variables for static linking
ENV RUSTFLAGS="-C target-feature=+crt-static"
ENV PKG_CONFIG_ALL_STATIC=1
ENV PKG_CONFIG_ALL_DYNAMIC=0

# Build the daemon binary in release mode with static linking
RUN cargo build --release --target x86_64-unknown-linux-musl --package daemon

# Runtime stage - using Alpine for minimal image with shell and directory support
FROM alpine:latest AS runtime

# Install CA certificates for HTTPS requests
RUN apk --no-cache add ca-certificates

# Create the app directory
RUN mkdir -p /app

# Copy the statically linked binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/daemon /app/daemon

# Make the binary executable
RUN chmod +x /app/daemon

# Set working directory
WORKDIR /app

# Set the binary as entrypoint
ENTRYPOINT ["/app/daemon"]
