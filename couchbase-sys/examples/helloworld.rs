extern crate couchbase_sys;

use couchbase_sys::*;
use std::ffi::CString;
use std::ptr;
use std::ffi::CStr;

/// Run with: export LCB_LOGLEVEL=5; cargo run --example=helloworld
fn main() {
    let connstr = CString::new("couchbase://localhost/default").unwrap();

    let mut cropts = lcb_create_st::default();
    cropts.v3.connstr = connstr.as_ptr();

    let mut instance: lcb_t = ptr::null_mut();
    unsafe {
        let res = lcb_create(&mut instance as *mut lcb_t, &cropts as *const lcb_create_st);
        println!("Create Res: {:?}", res);
        let res = lcb_connect(instance);
        println!("Connect Res: {:?}", res);
        let res = lcb_wait(instance);
        println!("Connect Wait Res: {:?}", res);
        let res = lcb_get_bootstrap_status(instance);
        println!(
            "Bootstrap Status: {:?} \"{}\"",
            res, // raw result enum
            CStr::from_ptr(lcb_strerror(instance, res)).to_str().unwrap() // description
        );
        lcb_destroy(instance);
    }
}
