#! /bin/bash

#export ANDROID_HOME=/Users/$USER/Library/Android/sdk
#export NDK_HOME=$ANDROID_HOME/ndk-bundle

#ndk-bundle/build/tools/make_standalone_toolchain.py --api 21 --arch arm64 --install-dir ndk-standalone-aarch64

export TARGET_AR=/home/mat/Android/Sdk/ndk-standalone-x86/bin/i686-linux-android-ar
export TARGET_CC=/home/mat/Android/Sdk/ndk-standalone-x86/bin/i686-linux-android-clang

cargo build --target=i686-linux-android --release
