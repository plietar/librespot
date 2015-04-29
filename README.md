# spotify-connect
This repository holds code and documentation about reverse engineering
Spotify Connect.

## Background
A while back, Spotify released a new service, Spotify Connect, which allows
spekers to stream music from Spotify, while being controlled from a smartphone.

This requires a special receiver, which are only sold by a couple of manufacturers.
Spotify have promised to release an SDK for a while now, but nothing was published.

## Goal
This project is an effort to try and reverse engineer Spotify Connect in order
to use it on any device, such as a Raspberry Pi or similar, both as a remote control
and as a receiver.

All firmwares have a `libspotify_embedded_shared.so` file in it, provided by
Spotify which contains the implementation of the protocol.
The first milestone is to understand the API exposed by this library, and write
our own code using it, either directly in C or using a wrapper for a higher
level language.

However this file is only available for armel and armhf, thus it cannot be run on
any platform we'd like to. (Note: qemu-user can be used to circumvent that)
The second milestone is to understand the whole protocol and reimplement it.
In addition to getting rid of a closed source blob, this would allow running on
virtually any platform.

## Status
The first milestone is pretty much reached. The `common/spotify.h` file contains
a description of the API exposed by the library. The `example` directory implements
a very basic receiver. A more complete receiver based on that header is being
developed here : [spotify-connect-web](https://github.com/Fornoth/spotify-connect-web).

The second milestone is much harder, since it involves rewriting everything from
scratch.
The top level key exchange and encryption is almost completly understood and can
be reproduced.
Several subsystems run on top of the connection, and they each need to be reverse
engineered. Hopefully the work for some of them has already been done by other
projects.
* Metadata: Only track metadata is implemented right now, but the rest should be
  straight forward. This is similar to the
  [websocket api](https://github.com/Hexxeh/spotify-websocket-api).
* Audio encryption and playback: The protocol is understood. The implementation
  works but needs to be improved. The protocol is identical to the one implemented
  by [despotify](http://sourceforge.net/p/despotify/code/HEAD/tree/)
* Connect: The protocol is more or less understood. The implementation is very basic
  for now. This part is completely new.

## Future plans
This repository currently only holds some experiments and POCs while I reverse the
protocol.

In the long term, I plan on writing a library callable from C or any other language
with an FFI, called `librespot`. This library will be backwards compatible with
spotify's official libspotify, but will provide extra features, such as Connect and
radios.

## Disclaimer
Using this code to connect to Spotify's API is probably forbidden by them, and might
result in you application key getting banned. Use at you own risk

## Links
* http://divideoverflow.com/2014/08/reversing-spotify-connect/
* https://github.com/sashahilton00/spotify-connect-resources
* http://sourceforge.net/p/despotify/code/HEAD/tree/
* https://github.com/Hexxeh/spotify-websocket-api

## Contact
Come and hang out on gitter if you need help or want to offer some.
https://gitter.im/sashahilton00/spotify-connect-resources

## License
Everything in this repository is licensed under the MIT license.
Some of the code requires `libspotify_embedded_shared.so` to run. This file is
the property of Spotify, and will not be shared here.

