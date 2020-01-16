use std::ptr;

use log::error;

use crate::sys::napi_value;
use crate::sys::napi_env;
use crate::sys::napi_callback_info;
use crate::sys::napi_ref;
use crate::val::JsEnv;
use crate::val::JsExports;
use crate::val::JsCallback;
use crate::NjError;
use crate::PropertiesBuilder;

pub trait JSClass: Sized {

    const CLASS_NAME: &'static str;

    // create my self from js callback
    fn crate_from_js(js_cb: &JsCallback) -> Result<Self,NjError>;


    fn set_wrapper(&mut self,wrapper: napi_ref);

    fn set_constructor(constructor: napi_ref);

    fn get_constructor() -> napi_ref;

    /// new instance
    fn new_instance(js_env: JsEnv, js_args: Vec<napi_value>) -> napi_value {

        let constructor = js_env.get_reference_value(Self::get_constructor());

        js_env.new_instance(constructor, js_args)
    }

    fn properties() -> PropertiesBuilder {
        vec![].into()
    }


    /// initialize class
    fn js_init(js_exports: &mut JsExports) {

        let properties = Self::properties();
        let js_constructor = js_exports.env().define_class(Self::CLASS_NAME,Self::js_new,properties);
        let js_ref = js_exports.env().create_reference(js_constructor, 1);
        Self::set_constructor(js_ref);
        js_exports.set_name_property(Self::CLASS_NAME,js_constructor);
    }

    
    
    /// Js constructor call
    #[no_mangle]
    extern "C" fn js_new(env: napi_env , info: napi_callback_info ) -> napi_value {

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

            let my_obj =  match Self::crate_from_js(&js_cb) {
                Ok(my_obj) => my_obj,
                Err(err) => {
                    error!("error creating js new: {}",err);
                    return ptr::null_mut()
                }
            };

            my_obj.wrap(js_env,js_cb)
        }
    }

    // wrap my self as JS object
    fn wrap(self, js_env: JsEnv, js_cb: JsCallback) -> napi_value {

        let boxed_self = Box::new(self);
        let raw_ptr = Box::into_raw(boxed_self);

        let wrap =  js_env.wrap(js_cb.this(),raw_ptr as *mut u8,Self::js_finalize);
       
        unsafe {
            let rust_ref: &mut Self = &mut * raw_ptr;
            rust_ref.set_wrapper(wrap);
        }

        js_cb.this_owned()
    }


    extern "C" fn js_finalize(_env: napi_env,finalize_data: *mut ::std::os::raw::c_void,
        _finalize_hint: *mut ::std::os::raw::c_void
    ) {

        println!("my object finalize");
        unsafe {
            let ptr: *mut Self = finalize_data as *mut Self;
            let _rust = Box::from_raw(ptr);
        }
        
    }

}