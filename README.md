This repository holds code and documentation for reverse enginnering
Spotify Connect.

# Background
A while back, Spotify released a new service, Spotify Connect, which allows
spekers to stream music from Spotify, while being controlled from a smartphone.

This requires a special receiver, which are only sold by a couple of manufacturers.
Spotify have promised to release an SDK for a while now, but nothing was published.

# Goal
This project is an effort to try and reverse enginner Spotify Connect in order
to use it on any device, such as a Raspberry Pi.

All firmwares have a libspotify_embedded_shared.so file in it, provided by
Spotify which contains the implementation of the protocol.
The first milestone is to understand the API exposed by this library, and write
our own code using it, either directly in C or using a wrapper for a higher
level language.

However this file being only for the armel architecture, it cannot be run on
any platform we'd like to. (Note: qemu can be used to circumvent that. More info
to come)
The second milestone is to understand the whole protocol and reimplement it.
In addition to getting rid of a closed source blob, this would allow running on
virtually any platform.

# Status
The first milestone is pretty much reached. The `common/spotify.h` file contains
a description of the API exposed by the library. The `example` directory implements
a very basic receiver.

The second milestone is much harder.
Understaning the protocol encryption is pretty much done (docs coming).
However there is a lot of packets exchanged over that encryption, and they each
need to be understood.
The audio is also encrypted, probably AES, so that needs to be figured out as
well. 

# Links
* http://divideoverflow.com/2014/08/reversing-spotify-connect/

  divideoverflow had done some initial research on it, but doesn't have any
  more time for it.

* https://github.com/sashahilton00/spotify-connect-resources

  sashahilton00 is centralizing information about Spotify Connect there.

* http://sourceforge.net/p/despotify/code/HEAD/tree/

  Despotify used to be an open source implementation of the Spotify protocol.
  It hasn't been updated in years and is out of date. However the encryption
  layer is extremely similar. Kudos to them for figuring it out.

# Contact
Come and hang out on gitter if you need help or want to offer some.
https://gitter.im/sashahilton00/spotify-connect-resources

# License
Everything in this repository is licensed under the MIT license.
Some of the code requires libspotify_embedded_shared.so to run. This file is
the property of Spotify, and will not be shared here.

