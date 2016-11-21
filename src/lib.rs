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
extern crate crypto;
extern crate hyper;
extern crate librespot_protocol as protocol;
extern crate lewton;
extern crate ogg;
extern crate num;
extern crate protobuf;
extern crate rand;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate shannon;
extern crate tokio_service;
extern crate url;
extern crate uuid;

#[cfg(not(target_os="windows"))]
extern crate mdns;

mod audio_backend;
mod audio_decrypt;
pub mod audio_key;
mod audio_file;
mod audio_queue;
mod broadcast;
pub mod channel;
mod diffie_hellman;
mod metadata;
mod ogg_async;
mod player;
mod session;
mod types;
mod util;
pub mod version;

// Some modules need to run through syntex on rust stable
// They are included from lib.in.rs
#[cfg(feature = "with-syntex")] include!(concat!(env!("OUT_DIR"), "/lib.rs"));
#[cfg(not(feature = "with-syntex"))] include!("lib.in.rs");

pub use session::Session;
pub use authentication::Credentials;
pub use spirc::SpircManager;
