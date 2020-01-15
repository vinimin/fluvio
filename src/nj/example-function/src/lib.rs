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
pub extern "C" fn hello_world (env: napi_env, cb_info: napi_callback_info) -> napi_value {

    let js_env = JsEnv::new(env); 
    let js_cb = js_env.get_cb_info(cb_info,1);    // a single argument
    let msg = match js_cb.get_value::<String>(0) {
        Ok(val) => val,
        Err(err) => {
            println!("error getting string argument: {}",err);
            return ptr::null_mut();
        }
    };
    let new_string = format!("{} world",msg);
    js_env.create_string_utf8(&new_string)
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
