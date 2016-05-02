#![allow(non_camel_case_types)]

extern crate libc;

use std::fmt;
use libc::{c_int, c_char, c_void};
use std::ffi::CStr;

#[repr(u32)]
#[derive(Debug,Clone,Copy)]
pub enum lcb_type_t {
    LCB_TYPE_BUCKET = 0,
    LCB_TYPE_CLUSTER = 1,
}

#[repr(u32)]
#[derive(Debug,Clone,Copy)]
pub enum lcb_error_t {
    LCB_SUCCESS = 0,
    LCB_AUTH_CONTINUE = 1,
    LCB_AUTH_ERROR = 2,
    LCB_DELTA_BADVAL = 3,
    LCB_E2BIG = 4,
    LCB_EBUSY = 5,
    LCB_EINTERNAL = 6,
    LCB_EINVAL = 7,
    LCB_ENOMEM = 8,
    LCB_ERANGE = 9,
    LCB_ERROR = 10,
    LCB_ETMPFAIL = 11,
    LCB_KEY_EEXISTS = 12,
    LCB_KEY_ENOENT = 13,
    LCB_DLOPEN_FAILED = 14,
    LCB_DLSYM_FAILED = 15,
    LCB_NETWORK_ERROR = 16,
    LCB_NOT_MY_VBUCKET = 17,
    LCB_NOT_STORED = 18,
    LCB_NOT_SUPPORTED = 19,
    LCB_UNKNOWN_COMMAND = 20,
    LCB_UNKNOWN_HOST = 21,
    LCB_PROTOCOL_ERROR = 22,
    LCB_ETIMEDOUT = 23,
    LCB_CONNECT_ERROR = 24,
    LCB_BUCKET_ENOENT = 25,
    LCB_CLIENT_ENOMEM = 26,
    LCB_CLIENT_ENOCONF = 27,
    LCB_EBADHANDLE = 28,
    LCB_SERVER_BUG = 29,
    LCB_PLUGIN_VERSION_MISMATCH = 30,
    LCB_INVALID_HOST_FORMAT = 31,
    LCB_INVALID_CHAR = 32,
    LCB_DURABILITY_ETOOMANY = 33,
    LCB_DUPLICATE_COMMANDS = 34,
    LCB_NO_MATCHING_SERVER = 35,
    LCB_BAD_ENVIRONMENT = 36,
    LCB_BUSY = 37,
    LCB_INVALID_USERNAME = 38,
    LCB_CONFIG_CACHE_INVALID = 39,
    LCB_SASLMECH_UNAVAILABLE = 40,
    LCB_TOO_MANY_REDIRECTS = 41,
    LCB_MAP_CHANGED = 42,
    LCB_INCOMPLETE_PACKET = 43,
    LCB_ECONNREFUSED = 44,
    LCB_ESOCKSHUTDOWN = 45,
    LCB_ECONNRESET = 46,
    LCB_ECANTGETPORT = 47,
    LCB_EFDLIMITREACHED = 48,
    LCB_ENETUNREACH = 49,
    LCB_ECTL_UNKNOWN = 50,
    LCB_ECTL_UNSUPPMODE = 51,
    LCB_ECTL_BADARG = 52,
    LCB_EMPTY_KEY = 53,
    LCB_SSL_ERROR = 54,
    LCB_SSL_CANTVERIFY = 55,
    LCB_SCHEDFAIL_INTERNAL = 56,
    LCB_CLIENT_FEATURE_UNAVAILABLE = 57,
    LCB_OPTIONS_CONFLICT = 58,
    LCB_HTTP_ERROR = 59,
    LCB_DURABILITY_NO_MUTATION_TOKENS = 60,
    LCB_UNKNOWN_MEMCACHED_ERROR = 61,
    LCB_MUTATION_LOST = 62,
    LCB_SUBDOC_PATH_ENOENT = 63,
    LCB_SUBDOC_PATH_MISMATCH = 64,
    LCB_SUBDOC_PATH_EINVAL = 65,
    LCB_SUBDOC_PATH_E2BIG = 66,
    LCB_SUBDOC_DOC_E2DEEP = 67,
    LCB_SUBDOC_VALUE_CANTINSERT = 68,
    LCB_SUBDOC_DOC_NOTJSON = 69,
    LCB_SUBDOC_NUM_ERANGE = 70,
    LCB_SUBDOC_BAD_DELTA = 71,
    LCB_SUBDOC_PATH_EEXISTS = 72,
    LCB_SUBDOC_MULTI_FAILURE = 73,
    LCB_SUBDOC_VALUE_E2DEEP = 74,
    LCB_EINVAL_MCD = 75,
    LCB_EMPTY_PATH = 76,
    LCB_UNKNOWN_SDCMD = 77,
    LCB_ENO_COMMANDS = 78,
    LCB_QUERY_ERROR = 79,
    LCB_MAX_ERROR = 4096,
}

impl fmt::Display for lcb_error_t {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let mut void_ptr = std::ptr::null_mut();
            write!(
                f,
                "{} ({:?})",
                CStr::from_ptr(lcb_strerror(void_ptr, *self)).to_str().unwrap(),
                self
            )
        }
    }
}


pub enum lcb_st { }
pub type lcb_t = *mut lcb_st;

#[repr(C)]
pub struct lcb_create_st {
    version: c_int,
    pub v3: lcb_create_st3,
}

impl Default for lcb_create_st {
    fn default() -> Self {
        lcb_create_st { version: 3, v3: lcb_create_st3::default() }
    }
}

#[repr(C)]
pub struct lcb_create_st3 {
    pub connstr: *const c_char,
    pub username: *const c_char,
    pub passwd: *const c_char,
    _pad_bucket: *mut c_void,
    io: *mut c_void,
    pub _type: lcb_type_t,
}

impl Default for lcb_create_st3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

extern {
    pub fn lcb_create(instance: *mut lcb_t, options: *const lcb_create_st) -> lcb_error_t;
    pub fn lcb_connect(instance: lcb_t) -> lcb_error_t;
    pub fn lcb_wait(instance: lcb_t) -> lcb_error_t;
    pub fn lcb_get_bootstrap_status(instance: lcb_t) -> lcb_error_t;
    pub fn lcb_destroy(instance: lcb_t);
    pub fn lcb_strerror(instance: lcb_t, error: lcb_error_t) -> *const c_char;
}
