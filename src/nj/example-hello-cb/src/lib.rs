use std::ptr;
use std::os::raw::c_char;


use libc::size_t;

use nj_sys::napi_status_napi_ok;
use nj_sys::napi_value;
use nj_sys::napi_env;
use nj_sys::napi_callback_info;
use nj_sys::napi_create_string_utf8;
use nj_sys::NAPI_AUTO_LENGTH;
use nj_sys::napi_get_cb_info;
use nj_sys::napi_get_global;
use nj_sys::napi_call_function;
use nj_sys::napi_create_function;
use nj_core::register_module;
use nj_core::define_property;
use nj_core::export_function;

#[no_mangle]
pub extern "C" fn hello_callback(env: napi_env,info: napi_callback_info) -> napi_value {
  
    let mut argc: size_t  = 1;

    let mut args: [napi_value; 1] = [ptr::null_mut(); 1];
    
    let status = unsafe { napi_get_cb_info(env, info, &mut argc,args.as_mut_ptr(), ptr::null_mut(), ptr::null_mut()) };
    assert_eq!(status,napi_status_napi_ok);
  
    let cb: napi_value = args[0];

    let mut label = ptr::null_mut();
    
    assert_eq!(unsafe { 
        napi_create_string_utf8(
            env,
            b"hello\x00" as *const u8 as *const c_char,
            NAPI_AUTO_LENGTH as usize, 
            &mut label) 
        },napi_status_napi_ok);
  
    
    let mut global = ptr::null_mut();
    let status = unsafe { napi_get_global(env, &mut global) };
    assert_eq!(status,napi_status_napi_ok);
  
    let mut result = ptr::null_mut();
    let status = unsafe { napi_call_function(env, global, cb, 1, &mut label, &mut result) };
    assert_eq!(status, napi_status_napi_ok);

    return ptr::null_mut()

}


export_function!(hello_callback);

register_module!("hello",init_hello);
