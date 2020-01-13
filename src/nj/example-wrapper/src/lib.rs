use std::ptr;


use nj_sys::napi_value;
use nj_sys::napi_env;
use nj_sys::napi_callback_info;
use nj_sys::napi_ref;
use nj_core::register_module;
use nj_core::define_property;
use nj_core::val::JsEnv;
use nj_core::val::JsExports;
use nj_core::PropertyBuilder;

#[no_mangle]
pub extern "C" fn hello_world (env: napi_env, _cb_info: napi_callback_info) -> napi_value {

    let js_env = JsEnv::new(env); 
    js_env.create_string_utf8("hello world")
}


static mut CONSTRUCTOR: napi_ref = ptr::null_mut();

#[no_mangle]
pub extern "C" fn init_my_object(env: napi_env , info: napi_callback_info ) -> napi_value {

    ptr::null_mut()
}

/// initialize class and register in exports
#[no_mangle]
pub extern "C" fn init_class(env: napi_env, exports: napi_value ) -> napi_value{

    let js_exports = JsExports::new(env,exports);

    let mut properties = vec![
            PropertyBuilder::new("test")
                .method(hello_world)
                .build()
    ];
    
    let js_class = js_exports.env().define_class("MyObject",hello_world,properties);
    let constructor = js_exports.env().create_reference(js_class, 1);
    js_exports.set_name_property("MyObject",js_class);


    return exports;
}



register_module!("hello",init_class);
