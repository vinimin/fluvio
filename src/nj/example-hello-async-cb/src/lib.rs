use std::ptr;
use std::time::Duration;

use nj_core::sys::napi_value;
use nj_core::sys::napi_env;
use nj_core::sys::napi_callback_info;


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
    data: *mut ::std::os::raw::c_void) {

    if env != ptr::null_mut() {

        let js_env = JsEnv::new(env);
        let global = js_env.get_global();

        let my_val: Box<f64> = unsafe { Box::from_raw(data as *mut f64) };

        let label = js_env.create_string_utf8("hello world");
        let value = js_env.create_double(*my_val);
        let _ = js_env.call_function(global,js_cb,vec![value,label]);
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

            // create new object 
            let my_val: f64 = 10.0;
            let my_box = Box::new(my_val);
            let ptr = Box::into_raw(my_box);

            xtsfn.call(Some(ptr as *mut core::ffi::c_void));
    });

    return ptr::null_mut()

  }



#[no_mangle]
pub extern "C" fn init_export (env: napi_env, exports: napi_value ) -> napi_value {
    
    define_property!("hello",env,exports,hello_callback_async);
    
    exports
}
  

register_module!("hello",init_export);
