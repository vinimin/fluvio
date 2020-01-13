use std::ptr;


use nj_core::sys::napi_value;
use nj_core::sys::napi_env;
use nj_core::sys::napi_callback_info;
use nj_core::register_module;
use nj_core::c_str;
use nj_core::val::JsEnv;
use nj_core::val::JsExports;
use nj_core::PropertyBuilder;

#[no_mangle]
pub extern "C" fn hello_world (env: napi_env, _cb_info: napi_callback_info) -> napi_value {

    let js_env = JsEnv::new(env); 
    js_env.create_string_utf8("hello world")
}



#[no_mangle]
pub extern "C" fn init_hello (env: napi_env, exports: napi_value ) -> napi_value{

    let js_exports = JsExports::new(env,exports);
    let prop = js_exports.prop_builder()
        .add(
            PropertyBuilder::new(c_str!("hello"))
                .method(hello_world)
                .build()
        ).build();
    
    js_exports.define_property(prop);

    return exports;
}



register_module!("hello",init_hello);
