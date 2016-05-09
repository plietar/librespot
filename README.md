# librespot
*librespot* is an open source client library for Spotify. It enables
applications to use Spotify's service, without using the official but
closed-source libspotify. Additionally, it will provide extra features
which are not available in the official library.

## Status
*librespot* is currently under development and is not fully functional yet. You
are however welcome to experiment with it.

## Building
Rust 1.7.0 or later is required to build librespot.

It also requires a C and C++ toolchain, with libprotoc and portaudio.

On debian / ubuntu, the following command will install these dependencies :
```shell
sudo apt-get install build-essential portaudio19-dev libprotoc-dev
```

On Fedora systems, the following command will install these dependencies :
```shell
sudo dnf install portaudio-devel protobuf-devel make gcc gcc-c++
```

On OS X, using homebrew :
```shell
brew install portaudio protobuf
```

Once you've cloned this repository you can build *librespot* using `cargo`.
```shell
cargo build --release
```

## Usage
A sample program implementing a headless Spotify Connect receiver is provided.
Once you've built *librespot*, run it using :
```shell
target/release/librespot --appkey APPKEY --username USERNAME --cache CACHEDIR --name DEVICENAME
```

## Discovery mode
*librespot* can be run in discovery mode, in which case no password is required at startup.
For that, simply omit the `--username` argument.

## Facebook Accounts
*librespot* can be built with Facebook authentication support. OpenSSL is required for this.

```shell
cargo build --release --features facebook
target/release/librespot --appkey APPKEY --cache CACHEDIR --name DEVICENAME --facebook
```

This will print a link to the console, which must be visited on the same computer *librespot* is running on.

## Audio Backends
*librespot* supports various audio backends. Multiple backends can be enabled at compile time by enabling the
corresponding cargo feature. By default, only PortAudio is enabled.

A specific backend can selected at runtime using the `--backend` switch.

```shell
cargo build --features portaudio-backend
target/release/librespot [...] --backend portaudio
```

The following backends are currently available :
- PortAudio 
- PulseAudio

## Development
When developing *librespot*, it is preferable to use Rust nightly, and build it using the following :
```shell
cargo build --no-default-features --features portaudio-backend
```

This produces better compilation error messages than with the default configuration.

## Disclaimer
Using this code to connect to Spotify's API is probably forbidden by them, and
might result in you application key getting banned. Use at you own risk

## Contact
Come and hang out on gitter if you need help or want to offer some.
https://gitter.im/sashahilton00/spotify-connect-resources

## License
Everything in this repository is licensed under the MIT license.

