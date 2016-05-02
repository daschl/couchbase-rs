extern crate couchbase_sys;

use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;
use std::ffi::CStr;

pub type CouchbaseError = couchbase_sys::lcb_error_t;

pub struct Cluster<'a> {
    hosts: &'a str,
    buckets: HashMap<&'a str, Bucket<'a>>,
}

impl<'a> Cluster<'a> {

    pub fn new(hosts: &'a str) -> Cluster {
        Cluster { hosts: hosts, buckets: HashMap::new() }
    }

    pub fn open_bucket(&mut self, name: &'a str, pass: &'a str) -> Result<&Bucket, CouchbaseError> {
        if !self.buckets.contains_key(&name) {
            let bucket = Bucket::open(self.hosts, name, pass);
            if bucket.is_ok() {
                self.buckets.insert(name, bucket.unwrap());
            } else {
                return Err(bucket.err().unwrap());
            }
        }
        Ok(self.buckets.get(&name).unwrap())
    }
}

pub struct Bucket<'a> {
    instance: couchbase_sys::lcb_t,
    name: &'a str,
    pass: &'a str,
}

impl<'a> Bucket<'a> {

    fn open(hosts: &'a str, name: &'a str, pass: &'a str) -> Result<Bucket<'a>, CouchbaseError> {
        let connstr = CString::new(format!("couchbase://{}/{}", hosts, name)).unwrap();
        let passstr = CString::new(pass).unwrap();

        let mut cropts = couchbase_sys::lcb_create_st::default();
        cropts.v3.connstr = connstr.as_ptr();
        cropts.v3.passwd = passstr.as_ptr();

        let mut instance: couchbase_sys::lcb_t = ptr::null_mut();
        let res = unsafe {
            couchbase_sys::lcb_create(
                &mut instance as *mut couchbase_sys::lcb_t,
                &cropts as *const couchbase_sys::lcb_create_st
            );
            couchbase_sys::lcb_connect(instance);
            couchbase_sys::lcb_wait(instance);
            couchbase_sys::lcb_get_bootstrap_status(instance)
        };

        match res {
            couchbase_sys::lcb_error_t::LCB_SUCCESS => Ok(Bucket { name: name, pass: pass, instance: instance }),
            e => Err(e)
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

}
