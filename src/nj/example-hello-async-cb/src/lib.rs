use std::ptr;
use std::time::Duration;

use nj_core::sys::napi_status_napi_ok;
use nj_core::sys::napi_value;
use nj_core::sys::napi_env;
use nj_core::sys::napi_callback_info;
use nj_core::sys::napi_call_threadsafe_function;
use nj_core::sys::napi_threadsafe_function_call_mode_napi_tsfn_blocking;
use nj_core::sys::napi_release_threadsafe_function;
use nj_core::sys::napi_threadsafe_function_release_mode_napi_tsfn_release;

use flv_future_core::spawn;
use flv_future_core::sleep;
use nj_core::register_module;
use nj_core::define_property;
use nj_core::val::JsEnv;

// convert the rust data into JS
pub extern "C" fn hello_callback_js(
    env: napi_env,
    js_cb: napi_value, 
    _context: *mut ::std::os::raw::c_void,
    _data: *mut ::std::os::raw::c_void) {

    if env != ptr::null_mut() {

        let js_env = JsEnv::new(env);
        let label = js_env.create_string_utf8("hello world");
        let global = js_env.get_global();

        let _ = js_env.call_function(global,js_cb,vec![label]);
    }
    
}


#[no_mangle]
pub extern "C" fn hello_callback_async(env: napi_env,info: napi_callback_info) -> napi_value {
  
    
    let js_env = JsEnv::new(env); 
    let js_cb = js_env.get_cb_info(info,1);    // only has 1 argument

    let xtsfn = js_cb.create_thread_safe_function("async",0,Some(hello_callback_js));


    spawn(async move {
            
            println!("sleeping");
            sleep(Duration::from_secs(1)).await;
            println!("woke from time");

            let inner_fn = xtsfn.inner();
            
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
pub extern "C" fn init_export (env: napi_env, exports: napi_value ) -> napi_value {
    
    define_property!("hello",env,exports,hello_callback_async);
    
    exports
}
  

register_module!("hello",init_export);
