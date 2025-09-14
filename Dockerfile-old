ARG BUILDPLATFORM

FROM --platform=$BUILDPLATFORM tonistiigi/xx AS xx

FROM --platform=$BUILDPLATFORM rust:trixie AS chef
COPY --from=xx / /

RUN apt-get update && apt-get install -y \
    clang \
    lld \
    pkg-config \
    musl-tools \
    musl-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# RUN cargo install cargo-chef 
WORKDIR /app

FROM chef AS depcacher
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo fetch

# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# COPY --from=planner /app/recipe.json recipe.json

# Setup the environment for the target platform
ARG TARGETPLATFORM
RUN xx-cargo --setup-target-triple

# Reuse the dockerfile for different crates
ARG PROJECT_NAME

# Build dependencies
# RUN --mount=type=cache,target=/usr/local/cargo/registry \
#     xx-cargo chef cook --release --recipe-path recipe.json

# Build the application
COPY . .
# RUN --mount=type=cache,target=/usr/local/cargo/registry \
#     --mount=type=cache,target=/app/target \
#     xx-cargo build --release --package ${PROJECT_NAME}
RUN --mount=type=cache,target=/app/target \
    xx-cargo build --release --target $(xx-cargo --print-target-triple --musl) --package ${PROJECT_NAME}

# Copy the binary out
RUN mkdir -p /app/output
RUN cp target/$(xx-cargo --print-target-triple --musl)/release/${PROJECT_NAME} /app/output

# Verify it's static
RUN xx-verify --static /app/output/${PROJECT_NAME}

FROM scratch AS runtime
COPY --from=builder /app/output /app
ENTRYPOINT ["/app"]
