use std::ptr;


use nj_sys::napi_value;
use nj_sys::napi_env;
use nj_sys::napi_callback_info;
use nj_sys::napi_create_string_utf8;
use nj_core::register_module;
use nj_core::define_property;
use nj_core::create_string_utf8;

#[no_mangle]
pub extern "C" fn hello_world (env: napi_env, _cb_info: napi_callback_info) -> napi_value {

    create_string_utf8!("hello world",env)
}



#[no_mangle]
pub extern "C" fn init_hello (env: napi_env, exports: napi_value ) -> napi_value{

    define_property!("hello",env,exports,hello_world);

    return exports;
}



register_module!("hello",init_hello);
