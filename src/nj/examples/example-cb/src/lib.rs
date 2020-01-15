use std::ptr;


use nj_core::sys::napi_value;
use nj_core::sys::napi_env;
use nj_core::sys::napi_callback_info;
use nj_core::register_module;
use nj_core::val::JsEnv;
use nj_core::val::JsExports;
use nj_core::Property;


#[no_mangle]
pub extern "C" fn hello_callback(env: napi_env,info: napi_callback_info) -> napi_value {
  
    let js_env = JsEnv::new(env);    
    let cb = js_env.get_cb_info(info,2);
   
    let first_arg = match cb.get_value::<f64>(0) {
        Ok(val) => val,
        Err(_err) =>  return ptr::null_mut()
    };
    let msg = format!("argument is: {}",first_arg);
    let label = js_env.create_string_utf8(&msg);
    let global = js_env.get_global();

    let cb_fn = cb.args(1);
    let _ = js_env.call_function(global, cb_fn,vec![label]);
    
    return ptr::null_mut()

}

#[no_mangle]
pub extern "C" fn init_export (env: napi_env, exports: napi_value ) -> napi_value {

    let js_exports = JsExports::new(env,exports);
    
    js_exports.define_property(
        js_exports.prop_builder()
            .add(
                Property::new("hello")
                    .method(hello_callback)
            ));
    
    exports
}


register_module!("hello",init_export);
