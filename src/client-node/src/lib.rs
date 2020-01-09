use std::ptr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::time::Duration;

use ctor::ctor;
use libc::size_t;

use nj_sys::napi_status_napi_ok;
use nj_sys::napi_value;
use nj_sys::napi_value__;
use nj_sys::napi_env;
use nj_sys::napi_status;
use nj_sys::napi_callback_info;
use nj_sys::napi_create_int32;
use nj_sys::napi_create_string_utf8;
use nj_sys::napi_property_descriptor;
use nj_sys::napi_property_attributes_napi_default;
use nj_sys::napi_define_properties;
use nj_sys::napi_module;
use nj_sys::NAPI_VERSION;
use nj_sys::NAPI_AUTO_LENGTH;
use nj_sys::napi_get_cb_info;
use nj_sys::napi_get_global;
use nj_sys::napi_call_function;
use nj_sys::napi_create_function;
use nj_sys::napi_create_threadsafe_function;
use nj_sys::napi_call_threadsafe_function;
use nj_sys::napi_threadsafe_function_call_mode_napi_tsfn_blocking;
use nj_sys::napi_threadsafe_function;
use nj_sys::napi_release_threadsafe_function;
use nj_sys::napi_threadsafe_function_release_mode_napi_tsfn_release;

use flv_future_core::spawn;
use flv_future_core::sleep;

#[no_mangle]
pub extern "C" fn hello_world (env: napi_env, cb_info: napi_callback_info) -> napi_value {

    let mut value = ptr::null_mut();

    let c_to_print = CString::new("Hello").expect("CString::new failed");

    let status = unsafe { napi_create_string_utf8(env,c_to_print.as_ptr(),NAPI_AUTO_LENGTH as usize, &mut value) };
    assert_eq!(status, napi_status_napi_ok);

    spawn(async {
        println!("sleeping");
        sleep(Duration::from_secs(3)).await;
        println!("woke from time");
    });

    return value   
}


#[no_mangle]
pub extern "C" fn hello_callback(env: napi_env,info: napi_callback_info) -> napi_value {
  
    let mut argc: size_t  = 1;

    let mut args: [napi_value; 1] = [ptr::null_mut(); 1];
    
    let status = unsafe { napi_get_cb_info(env, info, &mut argc,args.as_mut_ptr(), ptr::null_mut(), ptr::null_mut()) };
    assert_eq!(status,napi_status_napi_ok);
  
    let mut cb: napi_value = args[0];

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


// convert the rust data into JS
pub extern "C" fn hello_callback_js(
    env: napi_env,
    js_cb: napi_value, 
    context: *mut ::std::os::raw::c_void,
    data: *mut ::std::os::raw::c_void) {

    if (env != ptr::null_mut()) {

        let mut label = ptr::null_mut();
    
        assert_eq!(
            unsafe { 
                napi_create_string_utf8(
                env,
                b"hello\x00" as *const u8 as *const c_char,
                NAPI_AUTO_LENGTH as usize, 
                &mut label) 
            },
            napi_status_napi_ok);

        let mut global = ptr::null_mut();
        let status = unsafe { napi_get_global(env, &mut global) };
        assert_eq!(status,napi_status_napi_ok);

        let mut result = ptr::null_mut();
        assert_eq!(
            unsafe { 
                napi_call_function(
                    env, 
                    global, 
                    js_cb, 
                    1, 
                    &mut label, 
                    &mut result) 
            }, 
            napi_status_napi_ok);
    }
    

}

struct TsFn(napi_threadsafe_function);

unsafe impl Sync for TsFn{}
unsafe impl Send for TsFn{}

#[no_mangle]
pub extern "C" fn hello_callback_async(env: napi_env,info: napi_callback_info) -> napi_value {
  
    let mut argc: size_t  = 1;

    let mut args: [napi_value; 1] = [ptr::null_mut(); 1];

    let mut tsfn = ptr::null_mut();
    

    // retrieve callback function
    assert_eq!(
        unsafe { 
            napi_get_cb_info(
                env, 
                info, 
                &mut argc,
                args.as_mut_ptr(),
                ptr::null_mut(), 
                ptr::null_mut())
        }, 
        napi_status_napi_ok);

    let mut js_cb: napi_value = args[0];

    let mut work_name = ptr::null_mut();

    assert_eq!(
        unsafe {
            napi_create_string_utf8(
            env,
            b"async \x00" as *const u8 as *const c_char,
            NAPI_AUTO_LENGTH as usize,
            &mut work_name)
        },
        napi_status_napi_ok);

    // convert the callback
    assert_eq!(
        unsafe { 
            napi_create_threadsafe_function(
                env,
                js_cb,
                ptr::null_mut(),
                work_name,
                0,
                1,
                ptr::null_mut(),
                None,
                ptr::null_mut(),
                Some(hello_callback_js),
                &mut tsfn)
                },
        napi_status_napi_ok);


    let xtsfn = TsFn(tsfn);

    spawn(async move {
            
            println!("sleeping");
            sleep(Duration::from_secs(3)).await;
            println!("woke from time");

            let inner_fn = xtsfn.0;
            
            assert_eq!(
                unsafe {
                    napi_call_threadsafe_function(
                    inner_fn,
                    ptr::null_mut(),
                    napi_threadsafe_function_call_mode_napi_tsfn_blocking)
                    },
                napi_status_napi_ok);

            assert_eq!(
                unsafe {
                    napi_release_threadsafe_function(
                        inner_fn,
                        napi_threadsafe_function_release_mode_napi_tsfn_release)
                }, 
                napi_status_napi_ok)
            
        
    });

    return ptr::null_mut()

  }



#[no_mangle]
pub extern "C" fn init_hello_descriptor (env: napi_env, exports: napi_value ) -> napi_value{
    
    println!("registering hello module");
    
    let descriptor = napi_property_descriptor {
        utf8name: b"hello\x00" as *const u8 as *const c_char,
        name: ptr::null_mut(),
        method: Some(hello_world),
        getter: None,
        setter: None,
        value: ptr::null_mut(),
        attributes: napi_property_attributes_napi_default,
        data: ptr::null_mut()
    };
    
    let status = unsafe { napi_define_properties(env, exports, 1, &descriptor) };
    assert_eq!(status, napi_status_napi_ok);
    return exports;
}



#[no_mangle]
pub extern "C" fn init_hello (env: napi_env, exports: napi_value ) -> napi_value{
    
    println!("registering hello callback");
    
    let mut new_exports = ptr::null_mut();
    let status = unsafe { napi_create_function(env, b"x00" as *const u8 as *const c_char, NAPI_AUTO_LENGTH as usize,
         Some(hello_callback), ptr::null_mut(), &mut new_exports) };
  
    assert_eq!(status,napi_status_napi_ok);
    return new_exports;
}



#[no_mangle]
pub extern "C" fn init_async_hello_descriptor (env: napi_env, exports: napi_value ) -> napi_value{
    
    println!("registering hello module");

    let mut new_exports = ptr::null_mut();
    let status = unsafe { napi_create_function(env, b"x00" as *const u8 as *const c_char, NAPI_AUTO_LENGTH as usize,
         Some(hello_callback_async), ptr::null_mut(), &mut new_exports) };
  
    assert_eq!(status,napi_status_napi_ok);
    return new_exports;
    
}




#[ctor]
fn init_module() {

    extern "C" {
        pub fn napi_module_register(mod_: *mut napi_module);
    }

    static mut _module: napi_module  = napi_module {
        nm_version: NAPI_VERSION as i32,
        nm_flags: 0,
        nm_filename: b"test.rs\x00" as *const u8 as *const c_char,
        nm_register_func: Some(init_async_hello_descriptor),
        nm_modname:  b"hello\x00" as *const u8 as *const c_char,
        nm_priv: ptr::null_mut(),
        reserved: [ptr::null_mut(),ptr::null_mut(),ptr::null_mut(),ptr::null_mut()]
    };

    unsafe {
        println!("registering module version: {}",NAPI_VERSION);
        napi_module_register(&mut _module);
    }
   
}