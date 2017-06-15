#!/usr/bin/env bash

# Builds librespot for LibreELEC
# $ docker build -t librespot-cross -f contrib/Dockerfile .
# $ docker run -v /tmp/librespot-build:/build librespot-cross /src/contrib/docker-build-libreelec.sh

set -eux

for a in aarch64 arm x86_64; do
  mkdir -p /build/libreelec/$a/bin
  mkdir -p /build/libreelec/$a/lib
done

cargo build --release --no-default-features --features "alsa-backend with-avahi"
cp /build/release/librespot /build/libreelec/x86_64/bin
cp /usr/lib/x86_64-linux-gnu/libdns_sd.so.1 /build/libreelec/x86_64/lib

export PKG_CONFIG_ALLOW_CROSS=0

export PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig
cargo build --release --target aarch64-unknown-linux-gnu --no-default-features --features "alsa-backend with-avahi"
cp /build/aarch64-unknown-linux-gnu/release/librespot /build/libreelec/aarch64/bin
cp /usr/lib/aarch64-linux-gnu/libdns_sd.so.1 /build/libreelec/aarch64/lib

export PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig
cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features "alsa-backend with-avahi"
cp /build/arm-unknown-linux-gnueabihf/release/librespot /build/libreelec/arm/bin
cp /usr/lib/arm-linux-gnueabihf/libdns_sd.so.1 /build/libreelec/arm/lib
