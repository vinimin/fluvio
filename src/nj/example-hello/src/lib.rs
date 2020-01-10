use std::ptr;
use std::ffi::CString;

use nj_sys::napi_status_napi_ok;
use nj_sys::napi_value;
use nj_sys::napi_env;
use nj_sys::napi_callback_info;
use nj_sys::napi_create_string_utf8;
use nj_core::register_module;
use nj_core::define_property;

#[no_mangle]
pub extern "C" fn hello_world (env: napi_env, _cb_info: napi_callback_info) -> napi_value {

    let mut value = ptr::null_mut();

    let c_to_print = CString::new("Hello").expect("CString::new failed");

    let status = unsafe { napi_create_string_utf8(env,c_to_print.as_ptr(),5, &mut value) };
    assert_eq!(status, napi_status_napi_ok);

    return value   
}



#[no_mangle]
pub extern "C" fn init_hello (env: napi_env, exports: napi_value ) -> napi_value{

    define_property!("hello",env,exports,hello_world);

    return exports;
}



register_module!("hello",init_hello);
