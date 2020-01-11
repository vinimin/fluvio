#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod binding;

pub use binding::*;

unsafe impl Sync for binding::napi_module{}


pub type napi_callback_raw = unsafe extern "C" fn(env: crate::napi_env, info: crate::napi_callback_info) -> crate::napi_value;