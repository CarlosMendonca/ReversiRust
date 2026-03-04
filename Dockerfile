FROM docker.io/library/ubuntu:24.04

RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    curl \
    gcc-arm-none-eabi \
    gdb \
    git \
    libwayland-client0 \
    libwayland-egl1 \
    libegl1 \
    libwebkit2gtk-4.1-0 \
    libasound2t64 \
    libgl1-mesa-dri \
    libpulse0 \
    libudev-dev \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

ARG PLAYDATE_SDK_VERSION=3.0.3
RUN curl -fSL https://download-cdn.panic.com/playdate_sdk/Linux/PlaydateSDK-${PLAYDATE_SDK_VERSION}.tar.gz \
      -o /tmp/playdate-sdk.tar.gz \
    && mkdir -p /opt/playdate-sdk \
    && tar xzf /tmp/playdate-sdk.tar.gz -C /opt/playdate-sdk --strip-components=1 \
    && rm /tmp/playdate-sdk.tar.gz

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    --default-toolchain nightly-2025-04-25 \
    --profile minimal \
    --component rust-src,rustfmt \
    --target thumbv7em-none-eabihf \
    && . /root/.cargo/env \
    && cargo install cargo-playdate --version 0.5.8 --locked

ENV PLAYDATE_SDK_PATH=/opt/playdate-sdk
ENV PATH="/root/.cargo/bin:${PLAYDATE_SDK_PATH}/bin:${PATH}"

WORKDIR /workspace