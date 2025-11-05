# Multi-Architecture Dockerfile for Rust microbin project
# FIXED: Using debian:bookworm-slim instead of bitnami/minideb
# Supports: linux/amd64, linux/arm64, linux/arm/v7

FROM rust:latest AS build

WORKDIR /app

RUN \
  DEBIAN_FRONTEND=noninteractive \
  apt-get update &&\
  apt-get -y install ca-certificates tzdata

COPY . .

# Build natively for the target platform
# Docker Buildx will run this on the correct architecture via QEMU
RUN \
  CARGO_NET_GIT_FETCH_WITH_CLI=true \
  cargo build --release

# Use debian:bookworm-slim instead of bitnami/minideb
# debian:bookworm-slim supports: linux/amd64, linux/arm64, linux/arm/v7, linux/386
FROM debian:bookworm-slim

# microbin will be in /app
WORKDIR /app

# Install minimal required packages
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    tzdata && \
    rm -rf /var/lib/apt/lists/*

# Copy time zone info
COPY --from=build \
  /usr/share/zoneinfo \
  /usr/share/

COPY --from=build \
  /etc/ssl/certs/ca-certificates.crt \
  /etc/ssl/certs/ca-certificates.crt

# Copy built executable
COPY --from=build \
  /app/target/release/microbin \
  /usr/bin/microbin

# Expose webport used for the webserver to the docker runtime
EXPOSE 8080

ENTRYPOINT ["microbin"]
