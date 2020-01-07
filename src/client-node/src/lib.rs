use std::ptr;
use std::ffi::CString;

use nj_sys::napi_value;
use nj_sys::napi_env;
use nj_sys::napi_callback_info;
use nj_sys::napi_create_int32;
use nj_sys::napi_create_string_utf8;

#[no_mangle]
pub extern "C" fn hello_world (env: napi_env, cb_info: napi_callback_info) -> napi_value {

    let mut value = ptr::null_mut();

    let c_to_print = CString::new("Hello").expect("CString::new failed");

    let status = unsafe { napi_create_string_utf8(env,c_to_print.as_ptr(),5, &mut value) };

    return value   
}


