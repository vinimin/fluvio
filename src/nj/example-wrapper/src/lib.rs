use std::ptr;


use nj_core::sys::napi_value;
use nj_core::sys::napi_env;
use nj_core::sys::napi_callback_info;
use nj_core::sys::napi_ref;
use nj_core::sys::napi_property_descriptor;
use nj_core::c_str;
use nj_core::register_module;
use nj_core::val::JsEnv;
use nj_core::val::JsExports;
use nj_core::PropertyBuilder;
use nj_core::val::JsCallback;
use nj_core::JSClass;

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


    pub fn plus_one(&mut self) {
        self.val = self.val + 1.0;
    }

    pub fn value(&self) -> f64 {
        self.val
    }



    #[no_mangle]
    pub extern "C" fn js_plus_one(env: napi_env , info: napi_callback_info )  -> napi_value {

        println!("invoking plus one method");
    
        let js_env = JsEnv::new(env);

        let js_cb = js_env.get_cb_info(info,0); // there is no argument

        let my_obj = js_cb.unwrap::<MyObject>();

        my_obj.plus_one();

        let new_val = my_obj.value();
    
        js_env.create_double(my_obj.value())
    
    }


    #[no_mangle]
    pub extern "C" fn js_get_value(env: napi_env , info: napi_callback_info )  -> napi_value {

        println!("get value");

        let js_env = JsEnv::new(env);

        let js_cb = js_env.get_cb_info(info,0); // there is no argument
        let my_obj = js_cb.unwrap::<MyObject>();

        let new_val = my_obj.value();

        println!("rust object value is: {}",new_val);
    
        js_env.create_double(my_obj.value())
    }   

}

impl JSClass for MyObject {

    const CLASS_NAME: &'static str = "MyObject";

    fn crate_from_js(js_cb: &JsCallback) -> Box<Self> {

        let value = js_cb.get_value(0);

        println!("value passed: {}",value);

        Box::new(MyObject::new(value))

    }

    fn set_wrapper(&mut self,wrapper: napi_ref) {
        self.wrapper = wrapper;
    }



    fn properties() -> Vec<napi_property_descriptor> {
        vec![
            PropertyBuilder::new(c_str!("plusOne"))
                .method(Self::js_plus_one)
                .build(),
            PropertyBuilder::new(c_str!("value"))
                .getter(Self::js_get_value)
                .build()
        ]
    }


}


/// register all objects
#[no_mangle]
pub extern "C" fn init(env: napi_env, exports: napi_value ) -> napi_value{

    let mut js_exports = JsExports::new(env,exports);

    MyObject::js_init(&mut js_exports);

    return exports;
}



register_module!("hello",init);
