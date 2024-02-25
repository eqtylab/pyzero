FROM ubuntu:22.04

RUN apt-get update && \
    apt-get install --yes --no-install-recommends \
        ca-certificates \
        clang \
        curl \
        make \
        pkg-config \
        libssl-dev \
    ;

# Install Rust
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN cargo version

# Preload Rust Toolchain
COPY rust-toolchain /tmp/rust-toolchain
RUN rustup install $(cat /tmp/rust-toolchain) && rm /tmp/rust-toolchain

# Install RiscZero
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

ARG CARGO_RISCZERO_VERSION='0.20.1'
RUN cargo binstall -y "cargo-risczero@${CARGO_RISCZERO_VERSION}"

ARG RISC0_RUST_VERSION='v2024-02-08.1'
RUN cargo risczero install --version ${RISC0_RUST_VERSION}

# Use clang for c/cpp compilation
# (builds with CUDA enabled crash when building with gcc, didn't figure out why)
ENV CC=clang
ENV CXX=clang++
