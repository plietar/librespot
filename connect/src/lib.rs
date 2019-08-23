#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate base64;
extern crate futures;
extern crate hyper;
extern crate num_bigint;
extern crate protobuf;
extern crate rand;
extern crate tokio_core;
extern crate url;

extern crate sha1;
extern crate hmac;
extern crate aes_ctr;
extern crate block_modes;

#[cfg(feature = "with-dns-sd")]
extern crate dns_sd;

extern crate librespot_core as core;
extern crate librespot_playback as playback;
extern crate librespot_protocol as protocol;

pub mod context;
pub mod spirc;
