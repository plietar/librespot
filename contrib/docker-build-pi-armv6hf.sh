#!/usr/bin/env bash
set -eux

#remove debian armhf installed tools/libs
apt-get purge -y crossbuild-essential-armhf
apt-get purge -y *:armhf

# get raspberry pi tools (cross compiler etc); use fixed revision for now, should be updated sometimes
mkdir /pi-tools
curl -L https://github.com/raspberrypi/tools/archive/648a6eeb1e3c2b40af4eb34d88941ee0edeb3e9a.tar.gz | tar xz --strip-components 1 -C /pi-tools
export PATH=/pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/:$PATH
# create wrapper around gcc to point to rpi sysroot
echo -e '#!/bin/bash' > /pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/gcc-sysroot
echo -e "arm-linux-gnueabihf-gcc --sysroot /pi-tools/arm-bcm2708/arm-bcm2708hardfp-linux-gnueabi/arm-bcm2708hardfp-linux-gnueabi/sysroot \"\$@\"" >> /pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/gcc-sysroot
chmod +x /pi-tools/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64/bin/gcc-sysroot
# get alsa lib and headers
curl -OL http://mirrordirector.raspbian.org/raspbian/pool/main/a/alsa-lib/libasound2_1.0.25-4_armhf.deb
ar p libasound2_1.0.25-4_armhf.deb data.tar.gz | tar -xz -C /pi-tools/arm-bcm2708/arm-bcm2708hardfp-linux-gnueabi/arm-bcm2708hardfp-linux-gnueabi/sysroot
curl -OL http://mirrordirector.raspbian.org/raspbian/pool/main/a/alsa-lib/libasound2-dev_1.0.25-4_armhf.deb
ar p libasound2-dev_1.0.25-4_armhf.deb data.tar.gz | tar -xz -C /pi-tools/arm-bcm2708/arm-bcm2708hardfp-linux-gnueabi/arm-bcm2708hardfp-linux-gnueabi/sysroot/
# i don't why this is neccessary
ln -s ld-linux.so.3 /pi-tools/arm-bcm2708/arm-bcm2708hardfp-linux-gnueabi/arm-bcm2708hardfp-linux-gnueabi/sysroot/lib/ld-linux-armhf.so.3
# point cargo to use gcc wrapper as linker
echo -e '[target.arm-unknown-linux-gnueabihf]\nlinker = "gcc-sysroot"' > /.cargo/config

cargo build --release --target arm-unknown-linux-gnueabihf --no-default-features --features alsa-backend
