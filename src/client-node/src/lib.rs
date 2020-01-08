use std::ptr;
use std::ffi::CString;
use std::os::raw::c_char;

use nj_sys::napi_value;
use nj_sys::napi_env;
use nj_sys::napi_callback_info;
use nj_sys::napi_create_int32;
use nj_sys::napi_create_string_utf8;
use nj_sys::napi_property_descriptor;
use nj_sys::napi_property_attributes_napi_default;
use nj_sys::napi_define_properties;
use nj_sys::napi_module;
use nj_sys::NAPI_VERSION;

#[no_mangle]
pub extern "C" fn hello_world (env: napi_env, cb_info: napi_callback_info) -> napi_value {

    let mut value = ptr::null_mut();

    let c_to_print = CString::new("Hello").expect("CString::new failed");

    let status = unsafe { napi_create_string_utf8(env,c_to_print.as_ptr(),5, &mut value) };

    return value   
}

#[no_mangle]
pub extern "C" fn init_hello (env: napi_env, exports: napi_value ) -> napi_value{
    //
    
    let descriptor = napi_property_descriptor {
        utf8name:  CString::new("Hello").expect("CString::new failed").as_ptr(),
        name: ptr::null_mut(),
        method: Some(hello_world),
        getter: None,
        setter: None,
        value: ptr::null_mut(),
        attributes: napi_property_attributes_napi_default,
        data: ptr::null_mut()
    };
    
    let status = unsafe { napi_define_properties(env, exports, 1, &descriptor) };

    //napi_property_descriptor desc = DECLARE_NAPI_METHOD("hello", Method);
    //status = napi_define_properties(env, exports, 1, &desc);
    //assert(status == napi_ok);
    return exports;
  }

  pub static HelloModule: napi_module  = napi_module {
    nm_version: NAPI_VERSION as i32,
    nm_flags: 0,
    nm_filename: b"test.rs\x00" as *const u8 as *const libc::c_char as *const c_char,
    nm_register_func: Some(init_hello),
    nm_modname:  b"hello\x00" as *const u8 as *const libc::c_char as *const c_char,
    nm_priv: ptr::null_mut(),
    reserved: [ptr::null_mut(),ptr::null_mut(),ptr::null_mut(),ptr::null_mut()]

  };


