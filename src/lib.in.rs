#[macro_use] pub mod util;
mod album_cover;
pub mod apresolve;
mod audio_decrypt;
mod audio_file;
mod audio_key;
pub mod audio_backend;
pub mod authentication;
pub mod cache;
mod connection;
mod diffie_hellman;
pub mod mercury;
pub mod metadata;
pub mod player;
pub mod session;
pub mod spirc;
pub mod link;
pub mod stream;

#[cfg(feature = "facebook")]
pub mod spotilocal;

#[cfg(feature = "syslog-output")]
pub mod syslog_output;

pub use album_cover::get_album_cover;
