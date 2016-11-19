#![allow(dead_code, deprecated)]
#![crate_name = "librespot"]

#![cfg_attr(not(feature = "with-syntex"), feature(plugin, proc_macro))]
#![cfg_attr(not(feature = "with-syntex"), plugin(protobuf_macros))]
#![cfg_attr(not(feature = "with-syntex"), plugin(json_macros))]

#[macro_use] extern crate futures;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
#[macro_use] extern crate tokio_core as tokio;

#[cfg(not(feature = "with-syntex"))]
#[macro_use] extern crate serde_derive;

extern crate anymap;
extern crate bounded_spsc_queue;
extern crate byteorder;
extern crate coreaudio;
extern crate crypto;
extern crate ogg;
extern crate librespot_protocol as protocol;
extern crate lewton;
extern crate protobuf;
extern crate num;
extern crate rand;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate shannon;
extern crate uuid;

// include!/include_bytes! don't play nice with syntex, so place these here
pub mod version;

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

#[cfg(not(feature = "with-syntex"))]
include!("lib.in.rs");
