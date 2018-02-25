extern crate protobuf_macros;

use std::env;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").expect("path from environment OUT_DIR"));

    protobuf_macros::expand("src/lib.in.rs", &out.join("lib.rs")).expect("lib.rs expanded");

    println!("cargo:rerun-if-changed=src/lib.in.rs");
    println!("cargo:rerun-if-changed=src/spirc.rs");

}
