use std::ptr;


use nj_sys::napi_value;
use nj_sys::napi_env;
use nj_sys::napi_callback_info;
use nj_core::register_module;
use nj_core::define_property;
use nj_core::val::JsEnv;

#[no_mangle]
pub extern "C" fn hello_world (env: napi_env, _cb_info: napi_callback_info) -> napi_value {

    let js_env = JsEnv::new(env); 
    js_env.create_string_utf8("hello world")
}



#[no_mangle]
pub extern "C" fn init_hello (env: napi_env, exports: napi_value ) -> napi_value{

    napi_property_descriptor properties[] = {
        {"value", 0, 0, GetValue, SetValue, 0, napi_default, 0},
        DECLARE_NAPI_METHOD("plusOne", PlusOne),
        DECLARE_NAPI_METHOD("multiply", Multiply),
    };
  

    define_property!("hello",env,exports,hello_world);

    return exports;
}



register_module!("hello",init_hello);
