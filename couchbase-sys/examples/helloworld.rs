extern crate couchbase_sys;
extern crate libc;

use couchbase_sys::*;
use std::ffi::CString;
use std::ptr;
use std::ffi::CStr;

/// Run with: export LCB_LOGLEVEL=5; cargo run --example=helloworld
fn main() {
    let connstr = CString::new("couchbase://localhost/beer-sample").unwrap();

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
            res,
            CStr::from_ptr(lcb_strerror(instance, res)).to_str().unwrap() // description
        );

        lcb_install_callback3(instance, lcb_CALLBACKTYPE::LCB_CALLBACK_GET ,Some(op_callback));

        let key = "21st_amendment_brewery_cafe";
        let ckey = CString::new(key).unwrap();
        let mut gcmd = lcb_CMDGET::default();
        gcmd.key._type = lcb_KVBUFTYPE::LCB_KV_COPY;
        gcmd.key.contig.bytes = ckey.as_ptr() as *const libc::c_void;
        gcmd.key.contig.nbytes = key.len() as u64;

        let res = lcb_get3(instance, std::ptr::null(), &gcmd as *const lcb_CMDGET);
        println!("Get Res: {:?}", res);

        let res = lcb_wait(instance);
        println!("Get Wait Res: {:?}", res);

        lcb_destroy(instance);
    }
}

unsafe extern "C" fn op_callback(instance: lcb_t, cbtype: lcb_CALLBACKTYPE, resp: *const lcb_RESPBASE) {

    match cbtype {
        lcb_CALLBACKTYPE::LCB_CALLBACK_GET => {
            println!("> Get Callback!");
            let gresp = resp as *const lcb_RESPGET;
            println!(">> CAS: {}", (*gresp).cas);
            let res = CString::from_raw((*gresp).value as *mut i8);
            println!(">> Content: {}", res.into_string().unwrap());
        },
        _ => panic!("! Unknown Callback...")
    };

}
