use std::ptr;


use nj_sys::napi_value;
use nj_sys::napi_env;
use nj_sys::napi_callback_info;
use nj_sys::napi_ref;
use nj_core::register_module;
use nj_core::val::JsEnv;
use nj_core::val::JsExports;
use nj_core::PropertyBuilder;

struct MyObject {
    val: f64,
    wrapper: napi_ref
}

impl MyObject {
    pub fn new(val: f64) -> Self {
        Self {
            val,
            wrapper: ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn hello_world (env: napi_env, _cb_info: napi_callback_info) -> napi_value {

    let js_env = JsEnv::new(env); 
    js_env.create_string_utf8("hello world")
}


static mut CONSTRUCTOR: napi_ref = ptr::null_mut();

pub extern "C" fn finalize_my_object(env: napi_env,finalize_data: *mut ::std::os::raw::c_void,
    finalize_hint: *mut ::std::os::raw::c_void
) {

    println!("my object finalize");
}


#[no_mangle]
pub extern "C" fn init_my_object(env: napi_env , info: napi_callback_info ) -> napi_value {

    let js_env = JsEnv::new(env);
    let target = js_env.get_new_target(info);

    if target == ptr::null_mut() {
        // invokes as plain function

    } else {
        // Invoked as constructor: `new MyObject(...)`
        let js_cb = js_env.get_cb_info(info,1);

        let value = js_cb.get_value(0);

        let my_obj = Box::into_raw(Box::new(MyObject::new(value)));

        let wrap =  js_env.wrap(js_cb.this(),my_obj as *mut u8,finalize_my_object);
    }

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
