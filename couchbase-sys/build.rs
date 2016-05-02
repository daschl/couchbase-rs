extern crate cmake;
extern crate pkg_config;

use std::env;

fn main() {
    if env::var("COUCHBASE_SYS_USE_PKG_CONFIG").is_ok() {
        if pkg_config::find_library("libcouchbase").is_ok() {
            return
        }
    }

    let dst = cmake::build("libcouchbase");
    println!("cargo:rustc-link-lib=dylib=couchbase");
    println!("cargo:rustc-link-search={}", dst.join("lib").display());
}
