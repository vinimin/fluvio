use std::ptr;

use nj::sys::napi_value;
use nj::sys::napi_env;
use nj::sys::napi_callback_info;
use nj::sys::napi_ref;
use nj::core::register_module;
use nj::core::val::JsEnv;
use nj::core::val::JsExports;
use nj::core::Property;
use nj::core::val::JsCallback;
use nj::core::JSClass;
use nj::core::NjError;
use nj::core::PropertiesBuilder;

static mut MYOBJECT_CONSTRUCTOR: napi_ref = ptr::null_mut();

struct MyObject {
    val: f64,
}

impl MyObject {
    pub fn new(val: f64) -> Self {
        Self {
            val
        }
    }

    pub fn plus_one(&mut self) {
        self.val = self.val + 1.0;
    }

    pub fn value(&self) -> f64 {
        self.val
    }

    #[no_mangle]
    pub extern "C" fn js_plus_one(env: napi_env, info: napi_callback_info) -> napi_value {
        println!("invoking plus one method");

        let js_env = JsEnv::new(env);

        let js_cb = js_env.get_cb_info(info, 0); // there is no argument

        let my_obj = js_cb.unwrap::<MyObject>();

        my_obj.plus_one();

        js_env.create_double(my_obj.value())
    }

    #[no_mangle]
    pub extern "C" fn js_get_value(env: napi_env, info: napi_callback_info) -> napi_value {
        println!("get value");

        let js_env = JsEnv::new(env);

        let js_cb = js_env.get_cb_info(info, 0); // there is no argument
        let my_obj = js_cb.unwrap::<MyObject>();

        let new_val = my_obj.value();

        println!("rust object value is: {}", new_val);

        js_env.create_double(my_obj.value())
    }

    /// generates new object
    #[no_mangle]
    pub extern "C" fn js_multiply(env: napi_env, info: napi_callback_info) -> napi_value {
        let js_env = JsEnv::new(env);

        let js_cb = js_env.get_cb_info(info, 1); // a single argument
        let my_obj = js_cb.unwrap::<MyObject>();

        let arg_value = match js_cb.get_value::<f64>(0) {
            Ok(val) => val,
            Err(_err) => return ptr::null_mut(),
        };
        let my_val = my_obj.value();

        // multiply two values
        let new_val = js_env.create_double(arg_value * my_val);

        Self::new_instance(&js_env,vec![new_val])
    }
}

impl JSClass for MyObject {
    const CLASS_NAME: &'static str = "MyObject";

    fn create_from_js(js_cb: &JsCallback) -> Result<Self, NjError> {
        let value = js_cb.get_value::<f64>(0)?;

        println!("value passed: {}", value);

        Ok(MyObject::new(value))
    }


    fn set_constructor(constructor: napi_ref) {
        unsafe {
            MYOBJECT_CONSTRUCTOR = constructor;
        }
    }

    fn get_constructor() -> napi_ref {
        unsafe { MYOBJECT_CONSTRUCTOR }
    }

    fn properties() -> PropertiesBuilder {
        vec![
            Property::new("plusOne").method(Self::js_plus_one),
            Property::new("multiply").method(Self::js_multiply),
            Property::new("value").getter(Self::js_get_value),
        ]
        .into()
    }
}

/// register all objects
#[no_mangle]
pub extern "C" fn init(env: napi_env, exports: napi_value) -> napi_value {
    let mut js_exports = JsExports::new(env, exports);

    MyObject::js_init(&mut js_exports);

    return exports;
}

register_module!("hello", init);
