ARG BUILDPLATFORM

FROM --platform=$BUILDPLATFORM tonistiigi/xx AS xx

FROM --platform=$BUILDPLATFORM rust:slim AS chef
COPY --from=xx / /

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    clang \
    lld \
    perl \
    protobuf-compiler \
    libprotobuf-dev \
    cmake \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

FROM chef AS depcacher
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo fetch

FROM chef AS builder
RUN rustup target add x86_64-unknown-linux-musl
# Setup the environment for the target platform
ARG TARGETPLATFORM
RUN xx-cargo --setup-target-triple

ARG PROJECT_NAME
COPY . .

# Build the application
RUN --mount=type=cache,target=/app/target \
    xx-cargo build --release --package ${PROJECT_NAME}

# Copy the executable to an easily-findable path
RUN mkdir -p /app/target/release
RUN --mount=type=cache,target=/app/target \
    cp target/$(xx-cargo --print-target-triple)/release/${PROJECT_NAME} /app/output

# Verify the build
# RUN xx-verify --static /app/output

FROM debian:bookworm-slim AS runtime
COPY --from=builder /app/output /app
ENTRYPOINT ["/app"]
