#! /bin/bash

#export ANDROID_HOME=/Users/$USER/Library/Android/sdk
#export NDK_HOME=$ANDROID_HOME/ndk-bundle

#ndk-bundle/build/tools/make_standalone_toolchain.py --api 21 --arch arm64 --install-dir ndk-standalone-aarch64

export TARGET_AR=/home/mat/Android/Sdk/ndk-standalone-arm/bin/arm-linux-androideabi-ar
export TARGET_CC=/home/mat/Android/Sdk/ndk-standalone-arm/bin/arm-linux-androideabi-clang

cargo build --target=arm-linux-androideabi --release
