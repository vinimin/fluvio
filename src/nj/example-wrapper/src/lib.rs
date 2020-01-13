use std::ptr;


use nj_core::sys::napi_value;
use nj_core::sys::napi_env;
use nj_core::sys::napi_callback_info;
use nj_core::sys::napi_ref;
use nj_core::c_str;
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

    pub fn set_wrapper(&mut self,wrapper: napi_ref) {
        self.wrapper = wrapper;
    }

    pub fn plus_one(&mut self) {
        self.val = self.val + 1.0;
    }

    pub fn value(&self) -> f64 {
        self.val
    }


    /// initialize class
    pub fn js_init(js_exports: &mut JsExports) {

        let properties = vec![
                PropertyBuilder::new(c_str!("plusOne"))
                    .method(Self::js_plus_one)
                    .build(),
                PropertyBuilder::new(c_str!("value"))
                    .getter(Self::js_get_value)
                    .build()
        ];
        
        let js_constructor = js_exports.env().define_class("MyObject",MyObject::js_new,properties);
        let _js_ref = js_exports.env().create_reference(js_constructor, 1);
        js_exports.set_name_property("MyObject",js_constructor);
    }

    
    /// Js constructor call
    #[no_mangle]
    pub extern "C" fn js_new(env: napi_env , info: napi_callback_info ) -> napi_value {

        println!("MyObject constructor called");
        let js_env = JsEnv::new(env);
        let target = js_env.get_new_target(info);

        if target == ptr::null_mut() {
            // invokes as plain function
            ptr::null_mut()

        } else {
            println!("invoked as constructor");
            // Invoked as constructor: `new MyObject(...)`
            let js_cb = js_env.get_cb_info(info,1);

            let value = js_cb.get_value(0);

            println!("value passed: {}",value);

            let mut my_obj = Box::new(MyObject::new(value));
            let my_obj_raw_ptr: *mut Box<MyObject> = &mut my_obj;

            let wrap =  js_env.wrap(js_cb.this(),my_obj_raw_ptr as *mut u8,Self::js_finalize);
            my_obj.set_wrapper(wrap);

            Box::into_raw(my_obj);     // don't manage this object anymore

            js_cb.this_owned()
        }
    }



    #[no_mangle]
    pub extern "C" fn js_plus_one(env: napi_env , info: napi_callback_info )  -> napi_value {

        println!("invoking plus one method");
    
        let js_env = JsEnv::new(env);

        let js_cb = js_env.get_cb_info(info,0); // there is no argument

        let my_obj = js_cb.unwrap::<Box<MyObject>>();

        my_obj.plus_one();

        let new_val = my_obj.value();
    
        js_env.create_double(my_obj.value())
    
    }


    #[no_mangle]
    pub extern "C" fn js_get_value(env: napi_env , info: napi_callback_info )  -> napi_value {

        println!("get value");

        let js_env = JsEnv::new(env);

        let js_cb = js_env.get_cb_info(info,0); // there is no argument
        let my_obj = js_cb.unwrap::<Box<MyObject>>();

        let new_val = my_obj.value();
    
        js_env.create_double(my_obj.value())
    }   
    
    /*
    #[no_mange]
    pub extern "C" fn js_set_value(env: napi_env , info: napi_callback_info )  -> napi_value {

        println("set value");

        let js_env = JsEnv::new(env);

        let js_cb = js_env.get_cb_info(info,1); // there is no argument
        let my_obj = js_cb.unwrap::<Box<MyObject>>();

        let new_val = my_obj.value();
    
        js_env.create_double(my_obj.value())
    }  
    */



    pub extern "C" fn js_finalize(env: napi_env,finalize_data: *mut ::std::os::raw::c_void,
        finalize_hint: *mut ::std::os::raw::c_void
    ) {

        println!("my object finalize");
    }
    

}



static mut CONSTRUCTOR: napi_ref = ptr::null_mut();






/// register all objects
#[no_mangle]
pub extern "C" fn init(env: napi_env, exports: napi_value ) -> napi_value{

    let mut js_exports = JsExports::new(env,exports);

    MyObject::js_init(&mut js_exports);

    return exports;
}



register_module!("hello",init);
