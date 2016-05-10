#[macro_use]
extern crate log;
extern crate couchbase_sys;
extern crate libc;

use couchbase_sys::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;

pub type CouchbaseError = lcb_error_t;

pub struct Cluster<'a> {
    hosts: &'a str,
    buckets: HashMap<&'a str, Bucket<'a>>,
}

impl<'a> Cluster<'a> {

    pub fn new(hosts: &'a str) -> Cluster<'a> {
        Cluster { hosts: hosts, buckets: HashMap::new() }
    }

    pub fn from_localhost() -> Cluster<'a> {
        Cluster::new("127.0.0.1")
    }

    pub fn open_bucket(&mut self, name: &'a str, pass: &'a str) -> Result<&Bucket, CouchbaseError> {
        if !self.buckets.contains_key(&name) {
            let bucket = Bucket::open(self.hosts, name, pass);
            if bucket.is_ok() {
                info!("Opening Bucket \"{}\"", name);
                self.buckets.insert(name, bucket.unwrap());
            } else {
                return Err(bucket.err().unwrap());
            }
        } else {
            debug!("Bucket \"{}\" already opened, reusing instance.", name);
        }
        Ok(self.buckets.get(&name).unwrap())
    }
}

impl<'a> Drop for Cluster<'a> {
    fn drop(&mut self) {
        debug!("Couchbase Cluster goes out of scope (Drop).");
        for (name, bucket) in &mut self.buckets {
            debug!("Initiating close on bucket \"{}\"", name);
            bucket.close();
        }
    }
}

pub struct Bucket<'a> {
    instance: lcb_t,
    name: &'a str,
}

impl<'a> Bucket<'a> {

    fn open(hosts: &'a str, name: &'a str, pass: &'a str) -> Result<Bucket<'a>, CouchbaseError> {
        let connstr = CString::new(format!("couchbase://{}/{}", hosts, name)).unwrap();
        let passstr = CString::new(pass).unwrap();

        let mut cropts = lcb_create_st::default();
        cropts.v3.connstr = connstr.as_ptr();
        cropts.v3.passwd = passstr.as_ptr();

        let mut instance: lcb_t = ptr::null_mut();
        let res = unsafe {
            lcb_create(
                &mut instance as *mut lcb_t,
                &cropts as *const lcb_create_st
            );
            lcb_connect(instance);
            lcb_wait(instance);
            lcb_install_callback3(instance, lcb_CALLBACKTYPE::LCB_CALLBACK_GET ,Some(get_callback));
            lcb_get_bootstrap_status(instance)
        };

        match res {
            lcb_error_t::LCB_SUCCESS => Ok(Bucket { name: name, instance: instance }),
            e => Err(e)
        }
    }

    pub fn get(&self, id: &'a str) -> Result<Document<'a>, CouchbaseError> {
        let lcb_id = CString::new(id).unwrap();

        let mut cmd_get = lcb_CMDGET::default();
        cmd_get.key._type = lcb_KVBUFTYPE::LCB_KV_COPY;
        cmd_get.key.contig.bytes = lcb_id.as_ptr() as *const libc::c_void;
        cmd_get.key.contig.nbytes = id.len() as u64;

        let doc = Document { id: &id, cas: 0 };
        unsafe { lcb_get3(self.instance, &doc as *const Document as *const libc::c_void, &cmd_get as *const lcb_CMDGET); }
        unsafe { lcb_wait(self.instance); }

        Ok(doc)
    }

    pub fn close(&mut self) {
        info!("Closing Bucket \"{}\"", self.name);
        unsafe { lcb_destroy(self.instance); }
    }

    pub fn name(&self) -> &str {
        self.name
    }

}

pub struct Document<'a> {
    id: &'a str,
    cas: u64,
}

impl<'a> Document<'a> {
    pub fn cas(&self) -> u64 {
        self.cas
    }

    pub fn id(&self) -> &'a str {
        self.id
    }
}

unsafe extern "C" fn get_callback(instance: lcb_t, cbtype: lcb_CALLBACKTYPE, resp: *const lcb_RESPBASE) {
    let response = resp as *const lcb_RESPGET;

    let mut doc = (*response).cookie as *mut Document;
    (*doc).cas = (*response).cas;

    // set other stuff here of course
}
