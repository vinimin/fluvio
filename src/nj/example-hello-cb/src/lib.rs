use std::ptr;


use nj_core::sys::napi_value;
use nj_core::sys::napi_env;
use nj_core::sys::napi_callback_info;
use nj_core::register_module;
use nj_core::define_property;
use nj_core::val::JsEnv;

#[no_mangle]
pub extern "C" fn hello_callback(env: napi_env,info: napi_callback_info) -> napi_value {
  
    let js_env = JsEnv::new(env);    
    let cb = js_env.get_cb_info(info,2);
   
    let first_arg = cb.get_value(0);
    let msg = format!("argument is: {}",first_arg);
    let label = js_env.create_string_utf8(&msg);
    let global = js_env.get_global();

    let cb_fn = cb.args(1);
    let _ = js_env.call_function(global, cb_fn,vec![label]);
    
    return ptr::null_mut()

}

#[no_mangle]
pub extern "C" fn init_export (env: napi_env, exports: napi_value ) -> napi_value{

    define_property!("hello",env,exports,hello_callback);

    exports;
}


register_module!("hello",init_export);
