#!/usr/bin/env bash
set -eux

cargo build --release --no-default-features --features "alsa-backend with-external-mdns"
cp /usr/lib/x86_64-linux-gnu/libdns_sd.so.1 /build/release

export PKG_CONFIG_ALLOW_CROSS=0

export PKG_CONFIG_PATH=/usr/lib/aarch64-unknown-linux-gnu/pkgconfig
cargo build --release --target aarch64-unknown-linux-gnu --no-default-features --features "alsa-backend with-external-mdns"
cp /usr/lib/aarch64-linux-gnu/libdns_sd.so.1 /build/aarch64-unknown-linux-gnu/release

export PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabi/pkgconfig
cargo build --release --target arm-unknown-linux-gnueabi --no-default-features --features "alsa-backend with-external-mdns"
cp /usr/lib/arm-linux-gnueabi/libdns_sd.so.1 /build/arm-unknown-linux-gnueabi/release

export PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig
cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features "alsa-backend with-external-mdns"
cp /usr/lib/arm-linux-gnueabihf/libdns_sd.so.1 /build/arm-unknown-linux-gnueabihf/release

export PKG_CONFIG_PATH=/usr/lib/mipsel-linux-gnu/pkgconfig
cargo build --release --target mipsel-unknown-linux-gnu --no-default-features --features "alsa-backend with-external-mdns"
cp /usr/libmipsel-linux-gnu/libdns_sd.so.1 /build/mipsel-unknown-linux-gnu/release

