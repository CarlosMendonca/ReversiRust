FROM docker.io/library/ubuntu:24.04

RUN apt-get update && apt-get install -y \
    build-essential \
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
    && rm -rf /var/lib/apt/lists/*

ARG PLAYDATE_SDK_VERSION=3.0.3
RUN curl -fSL https://download-cdn.panic.com/playdate_sdk/Linux/PlaydateSDK-${PLAYDATE_SDK_VERSION}.tar.gz \
      -o /tmp/playdate-sdk.tar.gz \
    && mkdir -p /opt/playdate-sdk \
    && tar xzf /tmp/playdate-sdk.tar.gz -C /opt/playdate-sdk --strip-components=1 \
    && rm /tmp/playdate-sdk.tar.gz

ENV PLAYDATE_SDK_PATH=/opt/playdate-sdk
ENV PATH="${PLAYDATE_SDK_PATH}/bin:${PATH}"

WORKDIR /workspace